use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub aliases: Option<String>,
    pub duration: Option<i64>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub studio_id: Option<i64>,
    pub director: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCreate {
    pub name: String,
    pub aliases: Option<String>,
    pub duration: Option<i64>,
    pub date: Option<String>,
    pub rating: Option<i64>,
    pub studio_id: Option<i64>,
    pub director: Option<String>,
    pub description: Option<String>,
}
