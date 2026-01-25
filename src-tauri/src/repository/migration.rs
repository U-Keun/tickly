use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    migrate_add_category_id(conn)?;
    migrate_add_display_order_to_todos(conn)?;
    migrate_add_memo(conn)?;
    migrate_add_display_order_to_categories(conn)?;
    migrate_add_reminder_at(conn)?;
    Ok(())
}

fn migrate_add_category_id(conn: &Connection) -> Result<(), rusqlite::Error> {
    let column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='category_id'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = column_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN category_id INTEGER", [])?;
    }

    Ok(())
}

fn migrate_add_display_order_to_todos(conn: &Connection) -> Result<(), rusqlite::Error> {
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

    Ok(())
}

fn migrate_add_memo(conn: &Connection) -> Result<(), rusqlite::Error> {
    let memo_column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='memo'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = memo_column_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN memo TEXT", [])?;
    }

    Ok(())
}

fn migrate_add_display_order_to_categories(conn: &Connection) -> Result<(), rusqlite::Error> {
    let cat_order_column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('categories') WHERE name='display_order'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = cat_order_column_exists {
        conn.execute(
            "ALTER TABLE categories ADD COLUMN display_order INTEGER NOT NULL DEFAULT 0",
            [],
        )?;

        // Assign initial order based on id
        conn.execute("UPDATE categories SET display_order = id * 1000", [])?;
    }

    Ok(())
}

fn migrate_add_reminder_at(conn: &Connection) -> Result<(), rusqlite::Error> {
    let reminder_column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='reminder_at'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = reminder_column_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN reminder_at TEXT", [])?;
    }

    Ok(())
}
