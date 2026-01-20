use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use super::migration;

pub fn init_database(app: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

    let db_path = app_dir.join("tickly.db");
    let conn = Connection::open(db_path)?;

    // Create tables
    create_tables(&conn)?;

    // Run migrations
    migration::run_migrations(&conn)?;

    // Create default category if none exists
    create_default_category(&conn)?;

    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
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

    Ok(())
}

fn create_default_category(conn: &Connection) -> Result<(), rusqlite::Error> {
    use rusqlite::params;

    let category_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM categories",
        [],
        |row| row.get(0),
    )?;

    if category_count == 0 {
        conn.execute(
            "INSERT INTO categories (name, display_order) VALUES (?1, 1000)",
            params!["기본"],
        )?;
    }

    Ok(())
}
