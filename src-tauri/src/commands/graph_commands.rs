use tauri::State;

use super::with_db;
use crate::models::graph::GraphData;
use crate::repository::GraphRepository;
use crate::AppState;

#[tauri::command]
pub fn get_graph_data(state: State<AppState>) -> Result<GraphData, String> {
    with_db(&state, |db| GraphRepository::get_graph_data(db))
}
