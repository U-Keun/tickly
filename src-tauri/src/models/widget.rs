use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetTodoItem {
    pub id: i64,
    pub text: String,
    pub done: bool,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub display_order: i64,
    pub reminder_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetCategoryPendingItem {
    pub id: i64,
    pub text: String,
    pub display_order: i64,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetCategorySummary {
    pub category_id: Option<i64>,
    pub category_name: String,
    pub total_count: usize,
    pub pending_count: usize,
    pub first_pending_item_id: Option<i64>,
    pub pending_item_ids: Vec<i64>,
    pub pending_items: Vec<WidgetCategoryPendingItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetTheme {
    pub paper: String,
    pub canvas: String,
    pub stroke: String,
    pub ink: String,
    pub ink_muted: String,
    pub accent_sky: String,
    pub accent_sky_strong: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetSnapshot {
    pub generated_at: String,
    pub total_count: usize,
    pub pending_count: usize,
    pub items: Vec<WidgetTodoItem>,
    pub categories: Vec<WidgetCategorySummary>,
    pub theme: WidgetTheme,
}
