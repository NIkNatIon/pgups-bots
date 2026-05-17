#[derive(Debug, Clone)]
pub struct MenuNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub slug: String,
    pub image_url: Option<String>,
    pub sort_order: i32,
}
