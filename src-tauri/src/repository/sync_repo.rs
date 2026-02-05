use rusqlite::{params, Connection};

pub struct SyncRepository;

impl SyncRepository {
    pub fn get_metadata(conn: &Connection, key: &str) -> Result<Option<String>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT value FROM sync_metadata WHERE key = ?1")?;

        let mut rows = stmt.query_map(params![key], |row| row.get(0))?;

        if let Some(value) = rows.next() {
            Ok(Some(value?))
        } else {
            Ok(None)
        }
    }

    pub fn set_metadata(
        conn: &Connection,
        key: &str,
        value: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT OR REPLACE INTO sync_metadata (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_metadata(conn: &Connection, key: &str) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM sync_metadata WHERE key = ?1", params![key])?;
        Ok(())
    }

    pub fn get_last_synced_at(conn: &Connection) -> Result<Option<String>, rusqlite::Error> {
        Self::get_metadata(conn, "last_synced_at")
    }

    pub fn set_last_synced_at(conn: &Connection, value: &str) -> Result<(), rusqlite::Error> {
        Self::set_metadata(conn, "last_synced_at", value)
    }

    pub fn is_sync_enabled(conn: &Connection) -> Result<bool, rusqlite::Error> {
        let value = Self::get_metadata(conn, "sync_enabled")?;
        Ok(value.map(|v| v == "true").unwrap_or(false))
    }

    pub fn set_sync_enabled(conn: &Connection, enabled: bool) -> Result<(), rusqlite::Error> {
        Self::set_metadata(conn, "sync_enabled", if enabled { "true" } else { "false" })
    }

    pub fn get_pending_count(conn: &Connection) -> Result<usize, rusqlite::Error> {
        let todo_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM todos WHERE sync_status = 'pending' OR sync_status IS NULL",
            [],
            |row| row.get(0),
        )?;

        let category_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM categories WHERE sync_status = 'pending' OR sync_status IS NULL",
            [],
            |row| row.get(0),
        )?;

        Ok((todo_count + category_count) as usize)
    }
}
