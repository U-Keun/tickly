use tauri::State;

use super::with_db;
use crate::models::Category;
use crate::service::CategoryService;
use crate::AppState;

#[tauri::command]
pub fn get_categories(state: State<AppState>) -> Result<Vec<Category>, String> {
    with_db(&state, |db| CategoryService::get_all(db))
}

#[tauri::command]
pub fn add_category(name: String, state: State<AppState>) -> Result<Category, String> {
    with_db(&state, |db| CategoryService::create(db, &name))
}

#[tauri::command]
pub fn edit_category(id: i64, name: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| CategoryService::update(db, id, &name))
}

#[tauri::command]
pub fn delete_category(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| CategoryService::delete(db, id))
}

#[tauri::command]
pub fn reorder_categories(category_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| CategoryService::reorder(db, &category_ids))
}
