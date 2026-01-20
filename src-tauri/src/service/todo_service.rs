use rusqlite::Connection;

use crate::models::TodoItem;
use crate::repository::TodoRepository;

pub struct TodoService;

impl TodoService {
    pub fn get_items(
        conn: &Connection,
        category_id: Option<i64>,
    ) -> Result<Vec<TodoItem>, rusqlite::Error> {
        TodoRepository::get_by_category(conn, category_id)
    }

    pub fn create_item(
        conn: &Connection,
        text: &str,
        category_id: Option<i64>,
    ) -> Result<TodoItem, rusqlite::Error> {
        TodoRepository::create(conn, text, category_id)
    }

    pub fn toggle_item(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        TodoRepository::toggle(conn, id)
    }

    pub fn delete_item(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        TodoRepository::delete(conn, id)
    }

    pub fn update_text(conn: &Connection, id: i64, text: &str) -> Result<(), rusqlite::Error> {
        TodoRepository::update_text(conn, id, text)
    }

    pub fn update_memo(
        conn: &Connection,
        id: i64,
        memo: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        TodoRepository::update_memo(conn, id, memo)
    }

    pub fn reorder_items(conn: &Connection, item_ids: &[i64]) -> Result<(), rusqlite::Error> {
        TodoRepository::reorder(conn, item_ids)
    }
}
