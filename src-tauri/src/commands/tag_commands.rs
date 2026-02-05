use tauri::State;

use crate::models::{Tag, TodoItem};
use crate::service::TagService;
use crate::AppState;

#[tauri::command]
pub fn get_all_tags(state: State<AppState>) -> Result<Vec<Tag>, String> {
    let db = state.db.lock().unwrap();
    TagService::get_all_tags(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_tag(name: String, state: State<AppState>) -> Result<Tag, String> {
    let db = state.db.lock().unwrap();
    TagService::create_tag(&db, &name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_tag(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TagService::delete_tag(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_tag_to_item(
    item_id: i64,
    tag_name: String,
    state: State<AppState>,
) -> Result<Tag, String> {
    let db = state.db.lock().unwrap();
    TagService::add_tag_to_item(&db, item_id, &tag_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_tag_from_item(
    item_id: i64,
    tag_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TagService::remove_tag_from_item(&db, item_id, tag_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tags_for_item(item_id: i64, state: State<AppState>) -> Result<Vec<Tag>, String> {
    let db = state.db.lock().unwrap();
    TagService::get_tags_for_item(&db, item_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_items_by_tag(tag_id: i64, state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let db = state.db.lock().unwrap();
    TagService::get_items_by_tag(&db, tag_id).map_err(|e| e.to_string())
}
