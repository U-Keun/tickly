use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionLog {
    pub item_id: i64,
    pub completed_on: String,
    pub completed_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapData {
    pub item_id: i64,
    pub item_text: String,
    pub logs: Vec<CompletionLog>,
    pub total_days: i32,
    pub current_streak: i32,
    pub longest_streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedItem {
    pub id: i64,
    pub text: String,
    pub category_id: Option<i64>,
}
