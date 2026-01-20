use tauri::State;

use crate::models::TodoItem;
use crate::service::{ResetService, TodoService};
use crate::AppState;

#[tauri::command]
pub fn add_item(
    text: String,
    category_id: Option<i64>,
    state: State<AppState>,
) -> Result<TodoItem, String> {
    let db = state.db.lock().unwrap();
    TodoService::create_item(&db, &text, category_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_items(category_id: Option<i64>, state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let db = state.db.lock().unwrap();
    TodoService::get_items(&db, category_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_item(id: i64, state: State<AppState>) -> Result<(), String> {
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
