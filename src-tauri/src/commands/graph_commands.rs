use tauri::State;

use crate::models::graph::GraphData;
use crate::repository::GraphRepository;
use crate::AppState;

#[tauri::command]
pub fn get_graph_data(state: State<AppState>) -> Result<GraphData, String> {
    let db = state.db.lock().unwrap();
    GraphRepository::get_graph_data(&db).map_err(|e| e.to_string())
}
