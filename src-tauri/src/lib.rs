mod commands;
mod config;
mod db;
mod models;
mod services;

use db::{Database, default_db_path};
use models::app_state::AppState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = default_db_path();
    let database = Database::open(&db_path).expect("Failed to open database");
    let state = AppState::new(database);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::scene_commands::scene_list,
            commands::scene_commands::scene_find,
            commands::scene_commands::scene_create,
            commands::scene_commands::scene_update,
            commands::scene_commands::scene_destroy,
            commands::tag_commands::tag_list,
            commands::tag_commands::tag_find,
            commands::tag_commands::tag_create,
            commands::tag_commands::tag_destroy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
