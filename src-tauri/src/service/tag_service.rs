use rusqlite::Connection;

use crate::models::{Tag, TodoItem};
use crate::repository::{TagRepository, TodoRepository, TodoTagRepository};

pub struct TagService;

impl TagService {
    pub fn get_all_tags(conn: &Connection) -> Result<Vec<Tag>, rusqlite::Error> {
        TagRepository::get_all(conn)
    }

    /// Create a tag. If a tag with the same name exists, return the existing one.
    pub fn create_tag(conn: &Connection, name: &str) -> Result<Tag, rusqlite::Error> {
        let trimmed = name.trim();
        if let Some(existing) = TagRepository::get_by_name(conn, trimmed)? {
            return Ok(existing);
        }
        TagRepository::create(conn, trimmed)
    }

    pub fn delete_tag(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        if let Some(tag) = TagRepository::get_by_id(conn, id)? {
            if tag.sync_id.is_some() {
                TagRepository::mark_deleted(conn, id)
            } else {
                TagRepository::delete(conn, id)
            }
        } else {
            Ok(())
        }
    }

    /// Add a tag to an item. If the tag name doesn't exist, create it first.
    pub fn add_tag_to_item(
        conn: &Connection,
        item_id: i64,
        tag_name: &str,
    ) -> Result<Tag, rusqlite::Error> {
        let tag = Self::create_tag(conn, tag_name)?;
        TodoTagRepository::add_tag(conn, item_id, tag.id)?;
        TodoRepository::mark_updated(conn, item_id)?;
        Ok(tag)
    }

    pub fn remove_tag_from_item(
        conn: &Connection,
        item_id: i64,
        tag_id: i64,
    ) -> Result<(), rusqlite::Error> {
        // Check if the todo_tag has a sync_id (was synced)
        let has_sync_id = conn
            .query_row(
                "SELECT sync_id FROM todo_tags WHERE todo_id = ?1 AND tag_id = ?2",
                rusqlite::params![item_id, tag_id],
                |row| row.get::<_, Option<String>>(0),
            )
            .unwrap_or(None);

        if has_sync_id.is_some() {
            TodoTagRepository::mark_deleted(conn, item_id, tag_id)?;
        } else {
            TodoTagRepository::remove_tag(conn, item_id, tag_id)?;
        }
        TodoRepository::mark_updated(conn, item_id)?;
        Ok(())
    }

    pub fn get_tags_for_item(conn: &Connection, item_id: i64) -> Result<Vec<Tag>, rusqlite::Error> {
        TodoTagRepository::get_tags_for_item(conn, item_id)
    }

    /// Get all items that have a given tag (across all categories).
    pub fn get_items_by_tag(
        conn: &Connection,
        tag_id: i64,
    ) -> Result<Vec<TodoItem>, rusqlite::Error> {
        let todo_ids = TodoTagRepository::get_todo_ids_by_tag(conn, tag_id)?;
        let mut items = Vec::new();
        for id in todo_ids {
            if let Some(item) = TodoRepository::get_by_id(conn, id)? {
                items.push(item);
            }
        }
        Ok(items)
    }
}
