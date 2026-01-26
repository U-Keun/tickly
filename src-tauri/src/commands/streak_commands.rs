use tauri::State;

use crate::models::{HeatmapData, TrackedItem};
use crate::service::StreakService;
use crate::AppState;

#[tauri::command]
pub fn get_tracked_items(state: State<AppState>) -> Result<Vec<TrackedItem>, String> {
    let db = state.db.lock().unwrap();
    StreakService::get_tracked_items(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_item_heatmap_data(
    item_id: i64,
    state: State<AppState>,
) -> Result<Option<HeatmapData>, String> {
    let db = state.db.lock().unwrap();
    StreakService::get_item_heatmap_data(&db, item_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_track_streak(
    id: i64,
    track_streak: bool,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    StreakService::update_track_streak(&db, id, track_streak).map_err(|e| e.to_string())
}
