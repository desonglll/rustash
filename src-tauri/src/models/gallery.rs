use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gallery {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub code: Option<String>,
    pub photographer: Option<String>,
    pub url: Option<String>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub organized: bool,
    pub studio_id: Option<i64>,
    pub folder_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: i64,
    pub title: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
    pub photographer: Option<String>,
    pub url: Option<String>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub organized: bool,
    pub studio_id: Option<i64>,
    pub o_counter: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryChapter {
    pub id: i64,
    pub title: String,
    pub image_index: i64,
    pub gallery_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
