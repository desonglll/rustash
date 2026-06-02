use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub code: Option<String>,
    pub director: Option<String>,
    pub url: Option<String>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub organized: bool,
    pub studio_id: Option<i64>,
    pub resume_time: f64,
    pub play_duration: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCreate {
    pub title: Option<String>,
    pub details: Option<String>,
    pub code: Option<String>,
    pub director: Option<String>,
    pub url: Option<String>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub studio_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneUpdate {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub code: Option<String>,
    pub director: Option<String>,
    pub url: Option<String>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub organized: Option<bool>,
    pub studio_id: Option<i64>,
    pub resume_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMarker {
    pub id: i64,
    pub title: String,
    pub seconds: f64,
    pub end_seconds: Option<f64>,
    pub primary_tag_id: i64,
    pub scene_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
