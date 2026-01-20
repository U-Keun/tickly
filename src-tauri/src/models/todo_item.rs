use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: i64,
    pub text: String,
    pub done: bool,
    pub category_id: Option<i64>,
    pub display_order: i64,
    pub memo: Option<String>,
}
