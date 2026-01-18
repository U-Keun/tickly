use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Category {
    id: i64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoItem {
    id: i64,
    text: String,
    done: bool,
    category_id: Option<i64>,
    display_order: i64,
    memo: Option<String>,
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

    // Create categories table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Create todos table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0,
            category_id INTEGER,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // Create settings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    // Migration: Add category_id column if it doesn't exist
    let column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='category_id'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = column_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN category_id INTEGER", [])?;
    }

    // Migration: Add display_order column if it doesn't exist
    let order_column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='display_order'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = order_column_exists {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN display_order INTEGER NOT NULL DEFAULT 0",
            [],
        )?;

        // Assign initial order based on id (preserves current chronological order)
        conn.execute("UPDATE todos SET display_order = id * 1000", [])?;
    }

    // Migration: Add memo column if it doesn't exist
    let memo_column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='memo'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = memo_column_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN memo TEXT", [])?;
    }

    // Create default category if none exists
    let category_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM categories",
        [],
        |row| row.get(0),
    )?;

    if category_count == 0 {
        conn.execute("INSERT INTO categories (name) VALUES (?1)", params!["기본"])?;
    }

    Ok(conn)
}

#[tauri::command]
fn add_item(text: String, category_id: Option<i64>, state: State<AppState>) -> Result<TodoItem, String> {
    let db = state.db.lock().unwrap();

    // Calculate next display_order (max + 1000)
    let max_order: i64 = if let Some(cat_id) = category_id {
        db.query_row(
            "SELECT COALESCE(MAX(display_order), 0) FROM todos WHERE category_id = ?1",
            params![cat_id],
            |row| row.get(0),
        )
        .unwrap_or(0)
    } else {
        db.query_row(
            "SELECT COALESCE(MAX(display_order), 0) FROM todos",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0)
    };

    let display_order = max_order + 1000;

    db.execute(
        "INSERT INTO todos (text, done, category_id, display_order) VALUES (?1, 0, ?2, ?3)",
        params![text, category_id, display_order],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(TodoItem {
        id,
        text,
        done: false,
        category_id,
        display_order,
        memo: None,
    })
}

#[tauri::command]
fn get_items(category_id: Option<i64>, state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    let db = state.db.lock().unwrap();

    let mut items = Vec::new();

    if let Some(id) = category_id {
        let mut stmt = db
            .prepare("SELECT id, text, done, category_id, display_order, memo FROM todos WHERE category_id = ?1 ORDER BY done ASC, display_order ASC")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map(params![id], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    done: row.get(2)?,
                    category_id: row.get(3)?,
                    display_order: row.get(4)?,
                    memo: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?;

        for item in rows {
            items.push(item.map_err(|e| e.to_string())?);
        }
    } else {
        let mut stmt = db
            .prepare("SELECT id, text, done, category_id, display_order, memo FROM todos ORDER BY done ASC, display_order ASC")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    done: row.get(2)?,
                    category_id: row.get(3)?,
                    display_order: row.get(4)?,
                    memo: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?;

        for item in rows {
            items.push(item.map_err(|e| e.to_string())?);
        }
    }

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

#[tauri::command]
fn edit_item(id: i64, text: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE todos SET text = ?1 WHERE id = ?2",
        params![text, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn update_item_memo(id: i64, memo: Option<String>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE todos SET memo = ?1 WHERE id = ?2",
        params![memo, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_categories(state: State<AppState>) -> Result<Vec<Category>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, name FROM categories ORDER BY id")
        .map_err(|e| e.to_string())?;

    let categories = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(categories)
}

#[tauri::command]
fn add_category(name: String, state: State<AppState>) -> Result<Category, String> {
    let db = state.db.lock().unwrap();

    db.execute("INSERT INTO categories (name) VALUES (?1)", params![name])
        .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(Category { id, name })
}

#[tauri::command]
fn edit_category(id: i64, name: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    db.execute(
        "UPDATE categories SET name = ?1 WHERE id = ?2",
        params![name, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn delete_category(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    // Delete all todos in this category
    db.execute(
        "DELETE FROM todos WHERE category_id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;

    // Delete the category
    db.execute("DELETE FROM categories WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn reset_all_items(category_id: Option<i64>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    if let Some(id) = category_id {
        db.execute(
            "UPDATE todos SET done = 0 WHERE category_id = ?1",
            params![id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        db.execute("UPDATE todos SET done = 0", [])
            .map_err(|e| e.to_string())?;
    }

    // Update last reset date
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    db.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('last_reset_date', ?1)",
        params![today],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn check_and_auto_reset(state: State<AppState>) -> Result<bool, String> {
    let db = state.db.lock().unwrap();

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let last_reset: Result<String, _> = db.query_row(
        "SELECT value FROM settings WHERE key = 'last_reset_date'",
        [],
        |row| row.get(0),
    );

    match last_reset {
        Ok(last_date) => {
            if last_date != today {
                // New day, reset all items
                db.execute("UPDATE todos SET done = 0", [])
                    .map_err(|e| e.to_string())?;
                db.execute(
                    "INSERT OR REPLACE INTO settings (key, value) VALUES ('last_reset_date', ?1)",
                    params![today],
                )
                .map_err(|e| e.to_string())?;
                Ok(true) // Reset was performed
            } else {
                Ok(false) // No reset needed
            }
        }
        Err(_) => {
            // First time, set today as reset date
            db.execute(
                "INSERT INTO settings (key, value) VALUES ('last_reset_date', ?1)",
                params![today],
            )
            .map_err(|e| e.to_string())?;
            Ok(false)
        }
    }
}

#[tauri::command]
fn reorder_items(item_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    // Use transaction for atomic update
    db.execute("BEGIN TRANSACTION", [])
        .map_err(|e| e.to_string())?;

    for (index, item_id) in item_ids.iter().enumerate() {
        let new_order = (index as i64 + 1) * 1000;
        db.execute(
            "UPDATE todos SET display_order = ?1 WHERE id = ?2",
            params![new_order, item_id],
        )
        .map_err(|e| {
            let _ = db.execute("ROLLBACK", []);
            e.to_string()
        })?;
    }

    db.execute("COMMIT", []).map_err(|e| e.to_string())?;

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
        .invoke_handler(tauri::generate_handler![
            add_item,
            get_items,
            toggle_item,
            delete_item,
            edit_item,
            update_item_memo,
            get_categories,
            add_category,
            edit_category,
            delete_category,
            reset_all_items,
            check_and_auto_reset,
            reorder_items
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
