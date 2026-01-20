use rusqlite::Connection;

use crate::models::Category;
use crate::repository::CategoryRepository;

pub struct CategoryService;

impl CategoryService {
    pub fn get_all(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
        CategoryRepository::get_all(conn)
    }

    pub fn create(conn: &Connection, name: &str) -> Result<Category, rusqlite::Error> {
        CategoryRepository::create(conn, name)
    }

    pub fn update(conn: &Connection, id: i64, name: &str) -> Result<(), rusqlite::Error> {
        CategoryRepository::update(conn, id, name)
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        CategoryRepository::delete(conn, id)
    }

    pub fn reorder(conn: &Connection, category_ids: &[i64]) -> Result<(), rusqlite::Error> {
        CategoryRepository::reorder(conn, category_ids)
    }
}
