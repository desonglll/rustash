use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    pub id: i64,
    pub name: String,
    pub url: Option<String>,
    pub parent_id: Option<i64>,
    pub details: Option<String>,
    pub rating: Option<i64>,
    pub ignore_auto_tag: bool,
    pub favorite: bool,
    pub organized: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioCreate {
    pub name: String,
    pub url: Option<String>,
    pub parent_id: Option<i64>,
    pub details: Option<String>,
    pub rating: Option<i64>,
}
