use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub basename: String,
    pub parent_folder_id: i64,
    pub size: i64,
    pub mod_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: i64,
    pub path: String,
    pub parent_folder_id: Option<i64>,
    pub mod_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFile {
    pub file_id: i64,
    pub duration: f64,
    pub video_codec: String,
    pub format: String,
    pub audio_codec: String,
    pub width: i64,
    pub height: i64,
    pub frame_rate: f64,
    pub bit_rate: i64,
    pub interactive: bool,
    pub interactive_speed: Option<i64>,
}
