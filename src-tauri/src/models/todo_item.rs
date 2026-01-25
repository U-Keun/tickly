use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RepeatType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl Default for RepeatType {
    fn default() -> Self {
        RepeatType::None
    }
}

impl RepeatType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "daily" => RepeatType::Daily,
            "weekly" => RepeatType::Weekly,
            "monthly" => RepeatType::Monthly,
            _ => RepeatType::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            RepeatType::None => "none",
            RepeatType::Daily => "daily",
            RepeatType::Weekly => "weekly",
            RepeatType::Monthly => "monthly",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: i64,
    pub text: String,
    pub done: bool,
    pub category_id: Option<i64>,
    pub display_order: i64,
    pub memo: Option<String>,
    pub repeat_type: RepeatType,
    pub repeat_detail: Option<String>,
    pub next_due_at: Option<String>,
    pub last_completed_at: Option<String>,
}
