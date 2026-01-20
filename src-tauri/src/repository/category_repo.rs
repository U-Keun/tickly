use rusqlite::{params, Connection};

use crate::models::Category;

pub struct CategoryRepository;

impl CategoryRepository {
    pub fn get_all(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, display_order FROM categories ORDER BY display_order ASC",
        )?;

        let categories = stmt
            .query_map([], |row| {
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    display_order: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn create(conn: &Connection, name: &str) -> Result<Category, rusqlite::Error> {
        let max_order: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(display_order), 0) FROM categories",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let display_order = max_order + 1000;

        conn.execute(
            "INSERT INTO categories (name, display_order) VALUES (?1, ?2)",
            params![name, display_order],
        )?;

        let id = conn.last_insert_rowid();

        Ok(Category {
            id,
            name: name.to_string(),
            display_order,
        })
    }

    pub fn update(conn: &Connection, id: i64, name: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE categories SET name = ?1 WHERE id = ?2",
            params![name, id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        // Delete all todos in this category
        conn.execute("DELETE FROM todos WHERE category_id = ?1", params![id])?;

        // Delete the category
        conn.execute("DELETE FROM categories WHERE id = ?1", params![id])?;

        Ok(())
    }

    pub fn reorder(conn: &Connection, category_ids: &[i64]) -> Result<(), rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;

        for (index, category_id) in category_ids.iter().enumerate() {
            let new_order = (index as i64 + 1) * 1000;
            if let Err(e) = conn.execute(
                "UPDATE categories SET display_order = ?1 WHERE id = ?2",
                params![new_order, category_id],
            ) {
                let _ = conn.execute("ROLLBACK", []);
                return Err(e);
            }
        }

        conn.execute("COMMIT", [])?;

        Ok(())
    }
}
