use rusqlite::Connection;

use crate::models::{RepeatType, TodoItem};
use crate::repository::TodoRepository;
use crate::service::RepeatService;

pub struct TodoService;

impl TodoService {
    pub fn get_items(
        conn: &Connection,
        category_id: Option<i64>,
    ) -> Result<Vec<TodoItem>, rusqlite::Error> {
        TodoRepository::get_by_category(conn, category_id)
    }

    pub fn get_item(conn: &Connection, id: i64) -> Result<Option<TodoItem>, rusqlite::Error> {
        TodoRepository::get_by_id(conn, id)
    }

    pub fn create_item(
        conn: &Connection,
        text: &str,
        category_id: Option<i64>,
        repeat_type: &RepeatType,
        repeat_detail: Option<&str>,
        track_streak: bool,
    ) -> Result<TodoItem, rusqlite::Error> {
        // Calculate initial next_due_at for repeating items
        let next_due_at = if *repeat_type != RepeatType::None {
            let today = chrono::Local::now().date_naive();
            RepeatService::calculate_next_due(repeat_type, repeat_detail, today)
        } else {
            None
        };

        TodoRepository::create(
            conn,
            text,
            category_id,
            repeat_type,
            repeat_detail,
            next_due_at.as_deref(),
            track_streak,
        )
    }

    pub fn toggle_item(conn: &Connection, id: i64) -> Result<Option<TodoItem>, rusqlite::Error> {
        RepeatService::toggle_with_repeat(conn, id)
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

    pub fn update_repeat(
        conn: &Connection,
        id: i64,
        repeat_type: &RepeatType,
        repeat_detail: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        // Calculate next_due_at for the new repeat settings
        let next_due_at = if *repeat_type != RepeatType::None {
            let today = chrono::Local::now().date_naive();
            RepeatService::calculate_next_due(repeat_type, repeat_detail, today)
        } else {
            None
        };

        TodoRepository::update_repeat(conn, id, repeat_type, repeat_detail, next_due_at.as_deref())
    }

    pub fn reorder_items(conn: &Connection, item_ids: &[i64]) -> Result<(), rusqlite::Error> {
        TodoRepository::reorder(conn, item_ids)
    }
}
