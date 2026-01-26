use rusqlite::{params, Connection};

use crate::models::{RepeatType, TodoItem, TrackedItem};

pub struct TodoRepository;

impl TodoRepository {
    fn row_to_item(row: &rusqlite::Row) -> Result<TodoItem, rusqlite::Error> {
        let repeat_type_str: String = row.get(6)?;
        let track_streak_int: i32 = row.get(10)?;
        Ok(TodoItem {
            id: row.get(0)?,
            text: row.get(1)?,
            done: row.get(2)?,
            category_id: row.get(3)?,
            display_order: row.get(4)?,
            memo: row.get(5)?,
            repeat_type: RepeatType::from_str(&repeat_type_str),
            repeat_detail: row.get(7)?,
            next_due_at: row.get(8)?,
            last_completed_at: row.get(9)?,
            track_streak: track_streak_int != 0,
        })
    }

    pub fn get_by_category(
        conn: &Connection,
        category_id: Option<i64>,
    ) -> Result<Vec<TodoItem>, rusqlite::Error> {
        let mut items = Vec::new();

        if let Some(id) = category_id {
            let mut stmt = conn.prepare(
                "SELECT id, text, done, category_id, display_order, memo, repeat_type, repeat_detail, next_due_at, last_completed_at, track_streak FROM todos WHERE category_id = ?1 ORDER BY done ASC, display_order ASC",
            )?;

            let rows = stmt.query_map(params![id], Self::row_to_item)?;

            for item in rows {
                items.push(item?);
            }
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, text, done, category_id, display_order, memo, repeat_type, repeat_detail, next_due_at, last_completed_at, track_streak FROM todos ORDER BY done ASC, display_order ASC",
            )?;

            let rows = stmt.query_map([], Self::row_to_item)?;

            for item in rows {
                items.push(item?);
            }
        }

        Ok(items)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<TodoItem>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, text, done, category_id, display_order, memo, repeat_type, repeat_detail, next_due_at, last_completed_at, track_streak FROM todos WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map(params![id], Self::row_to_item)?;

        if let Some(item) = rows.next() {
            Ok(Some(item?))
        } else {
            Ok(None)
        }
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<TodoItem>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, text, done, category_id, display_order, memo, repeat_type, repeat_detail, next_due_at, last_completed_at, track_streak FROM todos ORDER BY display_order ASC",
        )?;

        let rows = stmt.query_map([], Self::row_to_item)?;
        let mut items = Vec::new();
        for item in rows {
            items.push(item?);
        }
        Ok(items)
    }

    pub fn get_tracked_items(conn: &Connection) -> Result<Vec<TrackedItem>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, text, category_id FROM todos WHERE track_streak = 1 ORDER BY display_order ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(TrackedItem {
                id: row.get(0)?,
                text: row.get(1)?,
                category_id: row.get(2)?,
            })
        })?;

        let mut items = Vec::new();
        for item in rows {
            items.push(item?);
        }
        Ok(items)
    }

    pub fn create(
        conn: &Connection,
        text: &str,
        category_id: Option<i64>,
        repeat_type: &RepeatType,
        repeat_detail: Option<&str>,
        next_due_at: Option<&str>,
    ) -> Result<TodoItem, rusqlite::Error> {
        let max_order: i64 = if let Some(cat_id) = category_id {
            conn.query_row(
                "SELECT COALESCE(MAX(display_order), 0) FROM todos WHERE category_id = ?1",
                params![cat_id],
                |row| row.get(0),
            )
            .unwrap_or(0)
        } else {
            conn.query_row(
                "SELECT COALESCE(MAX(display_order), 0) FROM todos",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0)
        };

        let display_order = max_order + 1000;
        let repeat_type_str = repeat_type.to_str();

        conn.execute(
            "INSERT INTO todos (text, done, category_id, display_order, repeat_type, repeat_detail, next_due_at) VALUES (?1, 0, ?2, ?3, ?4, ?5, ?6)",
            params![text, category_id, display_order, repeat_type_str, repeat_detail, next_due_at],
        )?;

        let id = conn.last_insert_rowid();

        Ok(TodoItem {
            id,
            text: text.to_string(),
            done: false,
            category_id,
            display_order,
            memo: None,
            repeat_type: repeat_type.clone(),
            repeat_detail: repeat_detail.map(|s| s.to_string()),
            next_due_at: next_due_at.map(|s| s.to_string()),
            last_completed_at: None,
            track_streak: false,
        })
    }

    pub fn update_track_streak(
        conn: &Connection,
        id: i64,
        track_streak: bool,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET track_streak = ?1 WHERE id = ?2",
            params![track_streak as i32, id],
        )?;
        Ok(())
    }

    pub fn toggle(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET done = NOT done WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn set_done(
        conn: &Connection,
        id: i64,
        done: bool,
        last_completed_at: Option<&str>,
        next_due_at: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET done = ?1, last_completed_at = ?2, next_due_at = ?3 WHERE id = ?4",
            params![done, last_completed_at, next_due_at, id],
        )?;
        Ok(())
    }

    pub fn update_repeat(
        conn: &Connection,
        id: i64,
        repeat_type: &RepeatType,
        repeat_detail: Option<&str>,
        next_due_at: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        let repeat_type_str = repeat_type.to_str();
        conn.execute(
            "UPDATE todos SET repeat_type = ?1, repeat_detail = ?2, next_due_at = ?3 WHERE id = ?4",
            params![repeat_type_str, repeat_detail, next_due_at, id],
        )?;
        Ok(())
    }

    pub fn reactivate(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET done = 0 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn update_text(conn: &Connection, id: i64, text: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET text = ?1 WHERE id = ?2",
            params![text, id],
        )?;
        Ok(())
    }

    pub fn update_memo(
        conn: &Connection,
        id: i64,
        memo: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET memo = ?1 WHERE id = ?2",
            params![memo, id],
        )?;
        Ok(())
    }

    pub fn reorder(conn: &Connection, item_ids: &[i64]) -> Result<(), rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;

        for (index, item_id) in item_ids.iter().enumerate() {
            let new_order = (index as i64 + 1) * 1000;
            if let Err(e) = conn.execute(
                "UPDATE todos SET display_order = ?1 WHERE id = ?2",
                params![new_order, item_id],
            ) {
                let _ = conn.execute("ROLLBACK", []);
                return Err(e);
            }
        }

        conn.execute("COMMIT", [])?;

        Ok(())
    }

    pub fn reset_all(conn: &Connection, category_id: Option<i64>) -> Result<(), rusqlite::Error> {
        if let Some(id) = category_id {
            conn.execute(
                "UPDATE todos SET done = 0 WHERE category_id = ?1",
                params![id],
            )?;
        } else {
            conn.execute("UPDATE todos SET done = 0", [])?;
        }
        Ok(())
    }
}
