use chrono::{Local, NaiveDate};
use rusqlite::Connection;

use crate::models::{CompletionLog, HeatmapData, TrackedItem};
use crate::repository::{CompletionLogRepository, TodoRepository};

pub struct StreakService;

impl StreakService {
    /// Log a completion for a specific item for today
    pub fn log_completion(conn: &Connection, item_id: i64) -> Result<(), rusqlite::Error> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        CompletionLogRepository::increment(conn, item_id, &today)
    }

    /// Remove a completion for a specific item for today (when unchecking)
    pub fn remove_completion(conn: &Connection, item_id: i64) -> Result<(), rusqlite::Error> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        CompletionLogRepository::decrement(conn, item_id, &today)
    }

    /// Get all tracked items
    pub fn get_tracked_items(conn: &Connection) -> Result<Vec<TrackedItem>, rusqlite::Error> {
        TodoRepository::get_tracked_items(conn)
    }

    /// Get heatmap data for a specific item for the last 365 days
    pub fn get_item_heatmap_data(
        conn: &Connection,
        item_id: i64,
    ) -> Result<Option<HeatmapData>, rusqlite::Error> {
        // Get item info
        let item = TodoRepository::get_by_id(conn, item_id)?;
        let Some(item) = item else {
            return Ok(None);
        };

        let logs = CompletionLogRepository::get_logs_for_item(conn, item_id, 365)?;
        let all_logs = CompletionLogRepository::get_all_logs_for_item(conn, item_id)?;

        let total_days = all_logs.len() as i32;
        let (current_streak, longest_streak) = Self::calculate_streaks(&all_logs);

        Ok(Some(HeatmapData {
            item_id,
            item_text: item.text,
            logs,
            total_days,
            current_streak,
            longest_streak,
        }))
    }

    /// Update track_streak setting for an item
    pub fn update_track_streak(
        conn: &Connection,
        item_id: i64,
        track_streak: bool,
    ) -> Result<(), rusqlite::Error> {
        TodoRepository::update_track_streak(conn, item_id, track_streak)?;

        Ok(())
    }

    /// Calculate current and longest streaks from completion logs
    fn calculate_streaks(logs: &[CompletionLog]) -> (i32, i32) {
        if logs.is_empty() {
            return (0, 0);
        }

        let today = Local::now().date_naive();
        let mut current_streak = 0;
        let mut longest_streak = 0;
        let mut temp_streak = 0;
        let mut prev_date: Option<NaiveDate> = None;

        for log in logs {
            if let Ok(date) = NaiveDate::parse_from_str(&log.completed_on, "%Y-%m-%d") {
                if let Some(prev) = prev_date {
                    let diff = (date - prev).num_days();
                    if diff == 1 {
                        // Consecutive day
                        temp_streak += 1;
                    } else if diff > 1 {
                        // Gap in streak
                        longest_streak = longest_streak.max(temp_streak);
                        temp_streak = 1;
                    }
                    // diff == 0 means same day, ignore
                } else {
                    temp_streak = 1;
                }
                prev_date = Some(date);
            }
        }

        // Check final streak
        longest_streak = longest_streak.max(temp_streak);

        // Calculate current streak (must include today or yesterday)
        if let Some(last_date) = prev_date {
            let days_since_last = (today - last_date).num_days();
            if days_since_last <= 1 {
                // Current streak is active (completed today or yesterday)
                current_streak = temp_streak;
            }
        }

        (current_streak, longest_streak)
    }
}
