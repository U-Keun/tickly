use rusqlite::{params, Connection};

use crate::models::TodoItem;

pub struct TodoRepository;

impl TodoRepository {
    pub fn get_by_category(
        conn: &Connection,
        category_id: Option<i64>,
    ) -> Result<Vec<TodoItem>, rusqlite::Error> {
        let mut items = Vec::new();

        if let Some(id) = category_id {
            let mut stmt = conn.prepare(
                "SELECT id, text, done, category_id, display_order, memo FROM todos WHERE category_id = ?1 ORDER BY done ASC, display_order ASC",
            )?;

            let rows = stmt.query_map(params![id], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    done: row.get(2)?,
                    category_id: row.get(3)?,
                    display_order: row.get(4)?,
                    memo: row.get(5)?,
                })
            })?;

            for item in rows {
                items.push(item?);
            }
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, text, done, category_id, display_order, memo FROM todos ORDER BY done ASC, display_order ASC",
            )?;

            let rows = stmt.query_map([], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    done: row.get(2)?,
                    category_id: row.get(3)?,
                    display_order: row.get(4)?,
                    memo: row.get(5)?,
                })
            })?;

            for item in rows {
                items.push(item?);
            }
        }

        Ok(items)
    }

    pub fn create(
        conn: &Connection,
        text: &str,
        category_id: Option<i64>,
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

        conn.execute(
            "INSERT INTO todos (text, done, category_id, display_order) VALUES (?1, 0, ?2, ?3)",
            params![text, category_id, display_order],
        )?;

        let id = conn.last_insert_rowid();

        Ok(TodoItem {
            id,
            text: text.to_string(),
            done: false,
            category_id,
            display_order,
            memo: None,
        })
    }

    pub fn toggle(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todos SET done = NOT done WHERE id = ?1",
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
