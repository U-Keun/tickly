use rusqlite::Connection;

use crate::repository::{SettingsRepository, TodoRepository};
use crate::service::repeat_service::get_logical_date;

pub struct ResetService;

impl ResetService {
    /// Reset all items in a category (or all items if category_id is None)
    /// and update the last reset date
    pub fn reset_items(
        conn: &Connection,
        category_id: Option<i64>,
    ) -> Result<(), rusqlite::Error> {
        TodoRepository::reset_all(conn, category_id)?;

        // Get reset time from settings and use logical date
        let reset_time = SettingsRepository::get(conn, "reset_time")?
            .unwrap_or_else(|| "00:00".to_string());
        let logical_date = get_logical_date(&reset_time);
        let today = logical_date.format("%Y-%m-%d").to_string();

        SettingsRepository::set(conn, "last_reset_date", &today)?;

        Ok(())
    }

    /// Check if a new day has started and auto-reset if needed
    /// Returns true if reset was performed, false otherwise
    pub fn check_and_auto_reset(conn: &Connection) -> Result<bool, rusqlite::Error> {
        // Get reset time from settings and use logical date
        let reset_time = SettingsRepository::get(conn, "reset_time")?
            .unwrap_or_else(|| "00:00".to_string());
        let logical_date = get_logical_date(&reset_time);
        let today = logical_date.format("%Y-%m-%d").to_string();

        let last_reset = SettingsRepository::get(conn, "last_reset_date")?;

        match last_reset {
            Some(last_date) => {
                if last_date != today {
                    // New day, reset all items
                    TodoRepository::reset_all(conn, None)?;
                    SettingsRepository::set(conn, "last_reset_date", &today)?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => {
                // First time, set today as reset date
                SettingsRepository::set(conn, "last_reset_date", &today)?;
                Ok(false)
            }
        }
    }
}
