mod commands;
mod models;
mod repository;
mod service;

use commands::*;
use repository::init_database;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<Connection>,
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
            // Todo commands
            add_item,
            get_items,
            toggle_item,
            delete_item,
            edit_item,
            update_item_memo,
            update_item_repeat,
            reorder_items,
            reset_all_items,
            check_and_auto_reset,
            process_repeats,
            // Category commands
            get_categories,
            add_category,
            edit_category,
            delete_category,
            reorder_categories,
            // Settings commands
            get_setting,
            set_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
