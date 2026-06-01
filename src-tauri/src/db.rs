// Database connection module
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub use crate::models::{Video, Tag, CreateVideoInput, CreateTagInput};
pub use crate::schema::run_migrations;

pub type DbPool = Arc<Mutex<SqlitePool>>;

/// Initialize the database connection
pub async fn init_database(app_data_dir: PathBuf) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    // Create data directory if it doesn't exist
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("rustash.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    log::info!("Connecting to database: {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;

    // Run migrations
    run_migrations(&pool).await?;

    log::info!("Database initialized successfully");

    Ok(pool)
}

/// Create a shared database pool
pub fn create_db_pool(pool: SqlitePool) -> DbPool {
    Arc::new(Mutex::new(pool))
}