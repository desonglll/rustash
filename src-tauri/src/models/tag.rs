use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub ignore_auto_tag: bool,
    pub favorite: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCreate {
    pub name: String,
    pub description: Option<String>,
    pub ignore_auto_tag: Option<bool>,
    pub favorite: Option<bool>,
}
