use tauri::{AppHandle, State};

use crate::models::WidgetSnapshot;
use crate::service::WidgetService;
use crate::AppState;

#[tauri::command]
pub fn get_widget_snapshot(
    max_items: Option<i64>,
    state: State<AppState>,
) -> Result<WidgetSnapshot, String> {
    let db = state.db.lock().unwrap();
    WidgetService::get_snapshot(&db, normalize_limit(max_items)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn refresh_widget_cache(
    max_items: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<WidgetSnapshot, String> {
    let db = state.db.lock().unwrap();
    WidgetService::refresh_cache(&db, &app, normalize_limit(max_items))
}

#[tauri::command]
pub fn toggle_item_from_widget(
    id: i64,
    max_items: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<WidgetSnapshot, String> {
    let db = state.db.lock().unwrap();
    WidgetService::toggle_item_and_refresh(&db, &app, id, normalize_limit(max_items))
}

#[tauri::command]
pub fn process_widget_actions(
    max_items: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    WidgetService::process_pending_actions(&db, &app, normalize_limit(max_items))
        .map(|processed| processed as i64)
}

#[tauri::command]
pub fn set_widget_cache_path(path: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    WidgetService::set_cache_path(&db, &path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_widget_app_group_id(app_group_id: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    WidgetService::set_app_group_id(&db, &app_group_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_widget_cache_path(app: AppHandle, state: State<AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    WidgetService::get_cache_path(&db, &app)
}

#[tauri::command]
pub fn get_widget_app_group_id(state: State<AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    WidgetService::get_app_group_id(&db)
}

fn normalize_limit(max_items: Option<i64>) -> Option<usize> {
    max_items.and_then(|value| {
        if value > 0 {
            Some(value as usize)
        } else {
            None
        }
    })
}
