// Settings module
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub ffmpeg_path: String,
}

pub type SettingsManager = Arc<Mutex<Settings>>;

/// Get settings from database
pub async fn get_settings(pool: &SqlitePool) -> Result<Settings, String> {
    let result = sqlx::query_as::<_, (String,)>("SELECT value FROM settings WHERE key = 'config'")
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Some((value,)) => serde_json::from_str(&value).map_err(|e| e.to_string()),
        None => Ok(Settings::default()),
    }
}

/// Save settings to database
pub async fn save_settings(pool: &SqlitePool, settings: &Settings) -> Result<(), String> {
    let value = serde_json::to_string(settings).map_err(|e| e.to_string())?;

    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('config', ?)")
        .bind(&value)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}