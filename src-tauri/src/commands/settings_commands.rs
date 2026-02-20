use tauri::State;

use super::with_db;
use crate::repository::SettingsRepository;
use crate::AppState;

#[tauri::command]
pub fn get_setting(key: String, state: State<AppState>) -> Result<Option<String>, String> {
    with_db(&state, |db| SettingsRepository::get(db, &key))
}

#[tauri::command]
pub fn set_setting(key: String, value: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| SettingsRepository::set(db, &key, &value))
}
