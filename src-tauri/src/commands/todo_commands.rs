use tauri::State;

use crate::models::{RepeatType, TodoItem};
use crate::service::{RepeatService, ResetService, TodoService};
use crate::AppState;

#[tauri::command]
pub fn add_item(
    text: String,
    category_id: Option<i64>,
    repeat_type: Option<String>,
    repeat_detail: Option<String>,
    track_streak: Option<bool>,
    reminder_at: Option<String>,
    state: State<AppState>,
) -> Result<TodoItem, String> {
    let db = state.db.lock().unwrap();
    let repeat = repeat_type
        .map(|s| RepeatType::from_str(&s))
        .unwrap_or(RepeatType::None);
    TodoService::create_item(&db, &text, category_id, &repeat, repeat_detail.as_deref(), track_streak.unwrap_or(false), reminder_at.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_item_reminder(
    id: i64,
    reminder_at: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TodoService::update_reminder(&db, id, reminder_at.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_items(category_id: Option<i64>, state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let db = state.db.lock().unwrap();
    TodoService::get_items(&db, category_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_item(id: i64, state: State<AppState>) -> Result<Option<TodoItem>, String> {
    let db = state.db.lock().unwrap();
    TodoService::toggle_item(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_item(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TodoService::delete_item(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn edit_item(id: i64, text: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TodoService::update_text(&db, id, &text).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_item_memo(id: i64, memo: Option<String>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TodoService::update_memo(&db, id, memo.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_item_repeat(
    id: i64,
    repeat_type: String,
    repeat_detail: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let repeat = RepeatType::from_str(&repeat_type);
    TodoService::update_repeat(&db, id, &repeat, repeat_detail.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reorder_items(item_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TodoService::reorder_items(&db, &item_ids).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_all_items(category_id: Option<i64>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    ResetService::reset_items(&db, category_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn check_and_auto_reset(state: State<AppState>) -> Result<bool, String> {
    let db = state.db.lock().unwrap();
    ResetService::check_and_auto_reset(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn process_repeats(state: State<AppState>) -> Result<i32, String> {
    let db = state.db.lock().unwrap();
    RepeatService::process_repeats(&db).map_err(|e| e.to_string())
}
