use tauri::State;

use crate::repository::SettingsRepository;
use crate::AppState;

#[tauri::command]
pub fn get_setting(key: String, state: State<AppState>) -> Result<Option<String>, String> {
    let db = state.db.lock().unwrap();
    SettingsRepository::get(&db, &key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(key: String, value: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    SettingsRepository::set(&db, &key, &value).map_err(|e| e.to_string())
}
