use tauri::State;

use crate::models::Category;
use crate::service::CategoryService;
use crate::AppState;

#[tauri::command]
pub fn get_categories(state: State<AppState>) -> Result<Vec<Category>, String> {
    let db = state.db.lock().unwrap();
    CategoryService::get_all(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_category(name: String, state: State<AppState>) -> Result<Category, String> {
    let db = state.db.lock().unwrap();
    CategoryService::create(&db, &name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn edit_category(id: i64, name: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    CategoryService::update(&db, id, &name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_category(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    CategoryService::delete(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reorder_categories(category_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    CategoryService::reorder(&db, &category_ids).map_err(|e| e.to_string())
}
