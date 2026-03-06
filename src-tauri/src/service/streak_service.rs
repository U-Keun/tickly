use chrono::{Datelike, Local, NaiveDate, NaiveTime};
use rusqlite::Connection;

use crate::models::{CompletionLog, HeatmapData, HeatmapIntensity, RepeatType, TrackedItem};
use crate::repository::{CompletionLogRepository, SettingsRepository, TodoRepository};

pub struct StreakService;

enum StreakCadence {
    Daily,
    Weekly([bool; 7]),
    Monthly([bool; 32]),
}

#[derive(Default)]
struct StreakStats {
    current_streak: i32,
    longest_streak: i32,
    current_streak_dates: Vec<NaiveDate>,
    longest_streak_dates: Vec<NaiveDate>,
}

impl StreakCadence {
    fn from_repeat(repeat_type: &RepeatType, repeat_detail: Option<&str>) -> Self {
        match repeat_type {
            RepeatType::None | RepeatType::Daily => Self::Daily,
            RepeatType::Weekly => {
                let values = Self::parse_repeat_values(repeat_detail);
                let mut weekdays = [false; 7];

                for value in values {
                    if value <= 6 {
                        weekdays[value as usize] = true;
                    }
                }

                if weekdays.iter().any(|enabled| *enabled) {
                    Self::Weekly(weekdays)
                } else {
                    Self::Daily
                }
            }
            RepeatType::Monthly => {
                let values = Self::parse_repeat_values(repeat_detail);
                let mut month_days = [false; 32];

                for value in values {
                    if (1..=31).contains(&value) {
                        month_days[value as usize] = true;
                    }
                }

                if month_days.iter().skip(1).any(|enabled| *enabled) {
                    Self::Monthly(month_days)
                } else {
                    Self::Daily
                }
            }
        }
    }

    fn parse_repeat_values(repeat_detail: Option<&str>) -> Vec<u32> {
        repeat_detail
            .and_then(|detail| serde_json::from_str::<Vec<u32>>(detail).ok())
            .unwrap_or_default()
    }

    fn is_scheduled_on(&self, date: NaiveDate) -> bool {
        match self {
            Self::Daily => true,
            Self::Weekly(weekdays) => weekdays[date.weekday().num_days_from_sunday() as usize],
            Self::Monthly(month_days) => month_days[date.day() as usize],
        }
    }

    fn next_scheduled_after(&self, date: NaiveDate) -> NaiveDate {
        if let Self::Daily = self {
            return date + chrono::Duration::days(1);
        }

        let mut candidate = date + chrono::Duration::days(1);
        for _ in 0..400 {
            if self.is_scheduled_on(candidate) {
                return candidate;
            }
            candidate += chrono::Duration::days(1);
        }

        date + chrono::Duration::days(1)
    }
}

impl StreakService {
    fn get_logical_date(conn: &Connection) -> Result<NaiveDate, rusqlite::Error> {
        let reset_time =
            SettingsRepository::get(conn, "reset_time")?.unwrap_or_else(|| "00:00".to_string());
        let now = Local::now();
        let today = now.date_naive();

        let parts: Vec<&str> = reset_time.split(':').collect();
        let hour: u32 = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
        let min: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

        let reset_time_today = NaiveTime::from_hms_opt(hour, min, 0).unwrap_or(NaiveTime::MIN);
        let current_time = now.time();

        if current_time < reset_time_today {
            Ok(today - chrono::Duration::days(1))
        } else {
            Ok(today)
        }
    }

    /// Log a completion for a specific item for today
    pub fn log_completion(conn: &Connection, item_id: i64) -> Result<(), rusqlite::Error> {
        let today = Self::get_logical_date(conn)?.format("%Y-%m-%d").to_string();
        CompletionLogRepository::increment(conn, item_id, &today)
    }

    /// Remove a completion for a specific item for today (when unchecking)
    pub fn remove_completion(conn: &Connection, item_id: i64) -> Result<(), rusqlite::Error> {
        let today = Self::get_logical_date(conn)?.format("%Y-%m-%d").to_string();
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
        let logical_today = Self::get_logical_date(conn)?;
        let cadence = StreakCadence::from_repeat(&item.repeat_type, item.repeat_detail.as_deref());
        let completion_dates = Self::scheduled_completion_dates(&all_logs, &cadence);
        let streak_segments = Self::build_streak_segments(&completion_dates, &cadence);
        let streak_stats = Self::calculate_streaks(&streak_segments, &cadence, logical_today);
        let combo_intensity = Self::build_combo_intensity(&streak_segments);

        let total_days = all_logs.len() as i32;

        Ok(Some(HeatmapData {
            item_id,
            item_text: item.text,
            logs,
            combo_intensity,
            total_days,
            current_streak: streak_stats.current_streak,
            longest_streak: streak_stats.longest_streak,
            current_streak_dates: Self::format_dates(&streak_stats.current_streak_dates),
            longest_streak_dates: Self::format_dates(&streak_stats.longest_streak_dates),
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
    fn calculate_streaks(
        streak_segments: &[Vec<NaiveDate>],
        cadence: &StreakCadence,
        today: NaiveDate,
    ) -> StreakStats {
        if streak_segments.is_empty() {
            return StreakStats::default();
        }

        let mut longest_segment = &streak_segments[0];
        for segment in streak_segments.iter().skip(1) {
            if segment.len() >= longest_segment.len() {
                longest_segment = segment;
            }
        }

        let current_segment = streak_segments.last().unwrap_or(longest_segment);
        let last_completion = *current_segment.last().unwrap_or(&today);

        // Current streak is active if at most one scheduled slot is pending.
        let next_expected = cadence.next_scheduled_after(last_completion);
        let current_streak_dates = if next_expected >= today {
            current_segment.clone()
        } else {
            Vec::new()
        };

        StreakStats {
            current_streak: current_streak_dates.len() as i32,
            longest_streak: longest_segment.len() as i32,
            current_streak_dates,
            longest_streak_dates: longest_segment.clone(),
        }
    }

    fn scheduled_completion_dates(
        logs: &[CompletionLog],
        cadence: &StreakCadence,
    ) -> Vec<NaiveDate> {
        logs.iter()
            .filter_map(|log| NaiveDate::parse_from_str(&log.completed_on, "%Y-%m-%d").ok())
            .filter(|date| cadence.is_scheduled_on(*date))
            .collect()
    }

    fn build_streak_segments(
        completion_dates: &[NaiveDate],
        cadence: &StreakCadence,
    ) -> Vec<Vec<NaiveDate>> {
        if completion_dates.is_empty() {
            return Vec::new();
        }

        let mut segments: Vec<Vec<NaiveDate>> = Vec::new();
        let mut current_segment = vec![completion_dates[0]];
        let mut prev_date = completion_dates[0];

        for date in completion_dates.iter().copied().skip(1) {
            let expected_next = cadence.next_scheduled_after(prev_date);
            if date == expected_next {
                current_segment.push(date);
            } else {
                segments.push(current_segment);
                current_segment = vec![date];
            }
            prev_date = date;
        }

        segments.push(current_segment);
        segments
    }

    fn build_combo_intensity(streak_segments: &[Vec<NaiveDate>]) -> Vec<HeatmapIntensity> {
        let mut intensities = Vec::new();

        for segment in streak_segments {
            for (index, date) in segment.iter().enumerate() {
                let combo_count = index + 1;
                let level = Self::combo_level_for_length(combo_count);
                intensities.push(HeatmapIntensity {
                    completed_on: date.format("%Y-%m-%d").to_string(),
                    level,
                });
            }
        }

        intensities
    }

    fn combo_level_for_length(length: usize) -> i32 {
        match length {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5..=6 => 5,
            7..=9 => 6,
            10..=13 => 7,
            14..=18 => 8,
            19..=25 => 9,
            _ => 10,
        }
    }

    fn format_dates(dates: &[NaiveDate]) -> Vec<String> {
        dates
            .iter()
            .map(|date| date.format("%Y-%m-%d").to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{StreakCadence, StreakService};
    use crate::models::{CompletionLog, RepeatType};
    use chrono::NaiveDate;

    fn log(date: &str) -> CompletionLog {
        CompletionLog {
            item_id: 1,
            completed_on: date.to_string(),
            completed_count: 1,
        }
    }

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn daily_streak_works_like_before() {
        let logs = vec![log("2026-03-01"), log("2026-03-02"), log("2026-03-03")];
        let cadence = StreakCadence::from_repeat(&RepeatType::Daily, None);
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);

        let stats = StreakService::calculate_streaks(&streak_segments, &cadence, date(2026, 3, 3));

        assert_eq!((stats.current_streak, stats.longest_streak), (3, 3));
        assert_eq!(
            stats.current_streak_dates,
            vec![date(2026, 3, 1), date(2026, 3, 2), date(2026, 3, 3)]
        );
    }

    #[test]
    fn weekly_streak_uses_expected_slots() {
        let logs = vec![log("2026-03-02"), log("2026-03-04")]; // Mon, Wed
        let cadence = StreakCadence::from_repeat(&RepeatType::Weekly, Some("[1,3,5]"));
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);

        let stats = StreakService::calculate_streaks(&streak_segments, &cadence, date(2026, 3, 6)); // Fri not done yet

        assert_eq!((stats.current_streak, stats.longest_streak), (2, 2));
        assert_eq!(
            stats.longest_streak_dates,
            vec![date(2026, 3, 2), date(2026, 3, 4)]
        );
    }

    #[test]
    fn weekly_streak_breaks_after_missing_expected_day() {
        let logs = vec![log("2026-03-02")]; // Mon
        let cadence = StreakCadence::from_repeat(&RepeatType::Weekly, Some("[1,3,5]"));
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);

        let stats = StreakService::calculate_streaks(&streak_segments, &cadence, date(2026, 3, 5)); // Thu after missing Wed

        assert_eq!((stats.current_streak, stats.longest_streak), (0, 1));
        assert!(stats.current_streak_dates.is_empty());
    }

    #[test]
    fn monthly_streak_skips_months_without_target_day() {
        let logs = vec![log("2026-01-31"), log("2026-03-31")];
        let cadence = StreakCadence::from_repeat(&RepeatType::Monthly, Some("[31]"));
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);

        let stats = StreakService::calculate_streaks(&streak_segments, &cadence, date(2026, 4, 1));

        assert_eq!((stats.current_streak, stats.longest_streak), (2, 2));
    }

    #[test]
    fn unscheduled_completions_do_not_count_for_repeating_streak() {
        let logs = vec![log("2026-03-03")]; // Tue
        let cadence = StreakCadence::from_repeat(&RepeatType::Weekly, Some("[1]")); // Mon only
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);

        let stats = StreakService::calculate_streaks(&streak_segments, &cadence, date(2026, 3, 4));

        assert_eq!((stats.current_streak, stats.longest_streak), (0, 0));
    }

    #[test]
    fn longest_streak_dates_use_latest_segment_when_tied() {
        let logs = vec![
            log("2026-03-02"),
            log("2026-03-03"),
            log("2026-03-06"),
            log("2026-03-07"),
        ];
        let cadence = StreakCadence::from_repeat(&RepeatType::Daily, None);
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);

        let stats = StreakService::calculate_streaks(&streak_segments, &cadence, date(2026, 3, 7));

        assert_eq!(
            stats.longest_streak_dates,
            vec![date(2026, 3, 6), date(2026, 3, 7)]
        );
    }

    #[test]
    fn combo_intensity_grows_within_a_combo_segment() {
        let logs = vec![
            log("2026-03-01"),
            log("2026-03-02"),
            log("2026-03-03"),
            log("2026-03-05"),
            log("2026-03-06"),
        ];
        let cadence = StreakCadence::from_repeat(&RepeatType::Daily, None);
        let completion_dates = StreakService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = StreakService::build_streak_segments(&completion_dates, &cadence);
        let combo_intensity = StreakService::build_combo_intensity(&streak_segments);

        let first_day_level = combo_intensity
            .iter()
            .find(|entry| entry.completed_on == "2026-03-01")
            .map(|entry| entry.level)
            .unwrap_or_default();
        let third_day_level = combo_intensity
            .iter()
            .find(|entry| entry.completed_on == "2026-03-03")
            .map(|entry| entry.level)
            .unwrap_or_default();
        let reset_day_level = combo_intensity
            .iter()
            .find(|entry| entry.completed_on == "2026-03-05")
            .map(|entry| entry.level)
            .unwrap_or_default();

        assert!(third_day_level > first_day_level);
        assert_eq!(reset_day_level, first_day_level);
    }
}
