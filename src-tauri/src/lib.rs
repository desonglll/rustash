// Main library entry point
mod db;
mod models;
mod schema;
mod scanner;

use db::{create_db_pool, init_database};
use models::{CreateTagInput, CreateVideoInput, Video, Tag};
use scanner::{ScanResult, extract_title};
use tauri::Manager;

pub type DbPool = db::DbPool;

// ============== Tauri Commands ==============

/// Get all videos
#[tauri::command]
async fn get_videos(pool: tauri::State<'_, DbPool>) -> Result<Vec<Video>, String> {
    let pool = pool.lock().await;
    let videos = sqlx::query_as::<_, Video>("SELECT * FROM videos ORDER BY created_at DESC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(videos)
}

/// Add a new video
#[tauri::command]
async fn add_video(pool: tauri::State<'_, DbPool>, input: CreateVideoInput) -> Result<Video, String> {
    let pool = pool.lock().await;
    let result = sqlx::query(
        "INSERT INTO videos (title, path, duration, width, height, size) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&input.title)
    .bind(&input.path)
    .bind(input.duration)
    .bind(input.width)
    .bind(input.height)
    .bind(input.size)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();

    let video = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(video)
}

/// Delete a video
#[tauri::command]
async fn delete_video(pool: tauri::State<'_, DbPool>, id: i64) -> Result<(), String> {
    let pool = pool.lock().await;
    sqlx::query("DELETE FROM videos WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Get all tags
#[tauri::command]
async fn get_tags(pool: tauri::State<'_, DbPool>) -> Result<Vec<Tag>, String> {
    let pool = pool.lock().await;
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(tags)
}

/// Add a new tag
#[tauri::command]
async fn add_tag(pool: tauri::State<'_, DbPool>, input: CreateTagInput) -> Result<Tag, String> {
    let pool = pool.lock().await;
    let result = sqlx::query("INSERT INTO tags (name) VALUES (?)")
        .bind(&input.name)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();

    let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(tag)
}

/// Delete a tag
#[tauri::command]
async fn delete_tag(pool: tauri::State<'_, DbPool>, id: i64) -> Result<(), String> {
    let pool = pool.lock().await;
    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Add tag to video
#[tauri::command]
async fn add_tag_to_video(pool: tauri::State<'_, DbPool>, video_id: i64, tag_id: i64) -> Result<(), String> {
    let pool = pool.lock().await;
    sqlx::query("INSERT OR IGNORE INTO video_tags (video_id, tag_id) VALUES (?, ?)")
        .bind(video_id)
        .bind(tag_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Remove tag from video
#[tauri::command]
async fn remove_tag_from_video(pool: tauri::State<'_, DbPool>, video_id: i64, tag_id: i64) -> Result<(), String> {
    let pool = pool.lock().await;
    sqlx::query("DELETE FROM video_tags WHERE video_id = ? AND tag_id = ?")
        .bind(video_id)
        .bind(tag_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Scan directory for video files
#[tauri::command]
async fn scan_directory(path: String) -> Result<ScanResult, String> {
    scanner::scan_directory(&path)
}

/// Import scanned files as videos
#[tauri::command]
async fn import_scanned_files(pool: tauri::State<'_, DbPool>, files: Vec<String>) -> Result<Vec<Video>, String> {
    let pool = pool.lock().await;
    let mut imported = Vec::new();

    for file_path in files {
        let title = scanner::extract_title(&file_path);
        let size = std::fs::metadata(&file_path)
            .map(|m| m.len() as i64)
            .unwrap_or(0);

        // Insert or ignore if already exists
        let result = sqlx::query(
            "INSERT OR IGNORE INTO videos (title, path, size) VALUES (?, ?, ?)"
        )
        .bind(&title)
        .bind(&file_path)
        .bind(size)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() > 0 {
            // Get the inserted video
            let video = sqlx::query_as::<_, Video>(
                "SELECT * FROM videos WHERE path = ?"
            )
            .bind(&file_path)
            .fetch_one(&*pool)
            .await
            .map_err(|e| e.to_string())?;
            imported.push(video);
        }
    }

    Ok(imported)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    log::info!("Starting rustash application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");

            // Initialize database
            let pool = tauri::async_runtime::block_on(async {
                init_database(app_data_dir).await.expect("Failed to initialize database")
            });

            let db_pool = create_db_pool(pool);
            app.manage(db_pool);

            log::info!("Application setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_videos,
            add_video,
            delete_video,
            get_tags,
            add_tag,
            delete_tag,
            add_tag_to_video,
            remove_tag_from_video,
            scan_directory,
            import_scanned_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}