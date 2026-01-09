use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoItem {
    id: i64,
    text: String,
    done: bool,
}

struct AppState {
    db: Mutex<Connection>,
}

fn init_database(app: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

    let db_path = app_dir.join("tickly.db");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;

    Ok(conn)
}

#[tauri::command]
fn add_item(text: String, state: State<AppState>) -> Result<TodoItem, String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "INSERT INTO todos (text, done) VALUES (?1, 0)",
        params![text],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(TodoItem {
        id,
        text,
        done: false,
    })
}

#[tauri::command]
fn get_items(state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, text, done FROM todos ORDER BY id")
        .map_err(|e| e.to_string())?;

    let items = stmt
        .query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                text: row.get(1)?,
                done: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(items)
}

#[tauri::command]
fn toggle_item(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE todos SET done = NOT done WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn delete_item(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute("DELETE FROM todos WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let conn = init_database(app.handle())?;
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_item, get_items, toggle_item, delete_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
