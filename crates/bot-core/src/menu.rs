use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
}
