use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    migrate_add_category_id(conn)?;
    migrate_add_display_order_to_todos(conn)?;
    migrate_add_memo(conn)?;
    migrate_add_display_order_to_categories(conn)?;
    migrate_add_reminder_at(conn)?;
    migrate_add_repeat_columns(conn)?;
    migrate_create_completion_logs(conn)?;
    migrate_add_track_streak(conn)?;
    migrate_completion_logs_add_item_id(conn)?;
    migrate_add_sync_fields(conn)?;
    migrate_create_auth_session(conn)?;
    migrate_create_sync_metadata(conn)?;
    migrate_create_tags(conn)?;
    migrate_create_todo_tags(conn)?;
    migrate_add_linked_app(conn)?;
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

fn migrate_add_repeat_columns(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add repeat_type column (none/daily/weekly/monthly)
    let repeat_type_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='repeat_type'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = repeat_type_exists {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN repeat_type TEXT NOT NULL DEFAULT 'none'",
            [],
        )?;
    }

    // Add repeat_detail column (JSON: weekly [0,3,5], monthly [1,15])
    let repeat_detail_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='repeat_detail'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = repeat_detail_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN repeat_detail TEXT", [])?;
    }

    // Add next_due_at column (YYYY-MM-DD format)
    let next_due_at_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='next_due_at'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = next_due_at_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN next_due_at TEXT", [])?;
    }

    // Add last_completed_at column (YYYY-MM-DD format)
    let last_completed_at_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='last_completed_at'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = last_completed_at_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN last_completed_at TEXT", [])?;
    }

    Ok(())
}

fn migrate_create_completion_logs(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS completion_logs (
            completed_on TEXT PRIMARY KEY,
            completed_count INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

fn migrate_add_track_streak(conn: &Connection) -> Result<(), rusqlite::Error> {
    let track_streak_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='track_streak'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = track_streak_exists {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN track_streak INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }

    Ok(())
}

fn migrate_completion_logs_add_item_id(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Check if item_id column already exists
    let item_id_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('completion_logs') WHERE name='item_id'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = item_id_exists {
        // Drop old table and create new one with item_id
        // (We don't need to preserve old data since we're switching to per-item tracking)
        conn.execute("DROP TABLE IF EXISTS completion_logs", [])?;
        conn.execute(
            "CREATE TABLE completion_logs (
                item_id INTEGER NOT NULL,
                completed_on TEXT NOT NULL,
                completed_count INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (item_id, completed_on)
            )",
            [],
        )?;
    }

    Ok(())
}

fn migrate_add_sync_fields(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add sync fields to todos table
    let sync_id_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='sync_id'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = sync_id_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN sync_id TEXT", [])?;
    }

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_todos_sync_id ON todos(sync_id)",
        [],
    )?;

    let created_at_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='created_at'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = created_at_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN created_at TEXT", [])?;
        conn.execute(
            "UPDATE todos SET created_at = datetime('now') WHERE created_at IS NULL",
            [],
        )?;
    }

    let updated_at_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='updated_at'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = updated_at_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN updated_at TEXT", [])?;
        conn.execute(
            "UPDATE todos SET updated_at = datetime('now') WHERE updated_at IS NULL",
            [],
        )?;
    }

    let sync_status_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='sync_status'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = sync_status_exists {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN sync_status TEXT DEFAULT 'pending'",
            [],
        )?;
    }

    // Add sync fields to categories table
    let cat_sync_id_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('categories') WHERE name='sync_id'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = cat_sync_id_exists {
        conn.execute("ALTER TABLE categories ADD COLUMN sync_id TEXT", [])?;
    }

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_categories_sync_id ON categories(sync_id)",
        [],
    )?;

    let cat_created_at_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('categories') WHERE name='created_at'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = cat_created_at_exists {
        conn.execute("ALTER TABLE categories ADD COLUMN created_at TEXT", [])?;
        conn.execute(
            "UPDATE categories SET created_at = datetime('now') WHERE created_at IS NULL",
            [],
        )?;
    }

    let cat_updated_at_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('categories') WHERE name='updated_at'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = cat_updated_at_exists {
        conn.execute("ALTER TABLE categories ADD COLUMN updated_at TEXT", [])?;
        conn.execute(
            "UPDATE categories SET updated_at = datetime('now') WHERE updated_at IS NULL",
            [],
        )?;

    }

    let cat_sync_status_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('categories') WHERE name='sync_status'",
        [],
        |row| row.get(0),
    );
    if let Ok(0) = cat_sync_status_exists {
        conn.execute(
            "ALTER TABLE categories ADD COLUMN sync_status TEXT DEFAULT 'pending'",
            [],
        )?;
    }

    Ok(())
}

fn migrate_create_auth_session(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS auth_session (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            user_id TEXT NOT NULL,
            access_token TEXT NOT NULL,
            refresh_token TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            provider TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn migrate_create_sync_metadata(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sync_metadata (
            key TEXT PRIMARY KEY,
            value TEXT
        )",
        [],
    )?;
    Ok(())
}

fn migrate_create_tags(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            sync_id TEXT,
            created_at TEXT,
            updated_at TEXT,
            sync_status TEXT DEFAULT 'pending'
        )",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_tags_sync_id ON tags(sync_id)",
        [],
    )?;
    Ok(())
}

fn migrate_add_linked_app(conn: &Connection) -> Result<(), rusqlite::Error> {
    let column_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('todos') WHERE name='linked_app'",
        [],
        |row| row.get(0),
    );

    if let Ok(0) = column_exists {
        conn.execute("ALTER TABLE todos ADD COLUMN linked_app TEXT", [])?;
    }

    Ok(())
}

fn migrate_create_todo_tags(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_tags (
            todo_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            sync_id TEXT,
            created_at TEXT,
            sync_status TEXT DEFAULT 'pending',
            PRIMARY KEY (todo_id, tag_id),
            FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_todo_tags_sync_id ON todo_tags(sync_id)",
        [],
    )?;
    Ok(())
}
