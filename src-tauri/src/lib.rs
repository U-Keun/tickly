use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoItem {
    id: u64,
    text: String,
    done: bool,
}

struct AppState {
    items: Mutex<Vec<TodoItem>>,
}

#[tauri::command]
fn add_item(text: String, state: State<AppState>) -> Result<TodoItem, String> {
    let mut items = state.items.lock().unwrap();
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let item = TodoItem {
        id,
        text,
        done: false,
    };

    items.push(item.clone());
    Ok(item)
}

#[tauri::command]
fn get_items(state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let items = state.items.lock().unwrap();
    Ok(items.clone())
}

#[tauri::command]
fn toggle_item(id: u64, state: State<AppState>) -> Result<(), String> {
    let mut items = state.items.lock().unwrap();
    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.done = !item.done;
        Ok(())
    } else {
        Err("Item not found".to_string())
    }
}

#[tauri::command]
fn delete_item(id: u64, state: State<AppState>) -> Result<(), String> {
    let mut items = state.items.lock().unwrap();
    items.retain(|item| item.id != id);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            items: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![add_item, get_items, toggle_item, delete_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
