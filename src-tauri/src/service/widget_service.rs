use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::models::{WidgetSnapshot, WidgetTodoItem};
use crate::repository::{CategoryRepository, SettingsRepository, TodoRepository};

pub struct WidgetService;

const WIDGET_CACHE_PATH_KEY: &str = "widget_cache_path";
const WIDGET_APP_GROUP_ID_KEY: &str = "widget_app_group_id";
const DEFAULT_WIDGET_APP_GROUP_ID: &str = "group.com.u-keunsong.tickly";
const DEFAULT_WIDGET_CACHE_FILE: &str = "widget-cache.json";
const DEFAULT_WIDGET_ITEM_LIMIT: usize = 20;
const MAX_WIDGET_ITEM_LIMIT: usize = 100;

impl WidgetService {
    pub fn get_snapshot(
        conn: &Connection,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, rusqlite::Error> {
        let mut todos = TodoRepository::get_all(conn)?;
        let categories = CategoryRepository::get_all(conn)?;
        let category_map: HashMap<i64, String> = categories
            .into_iter()
            .map(|cat| (cat.id, cat.name))
            .collect();

        todos.sort_by(|a, b| {
            a.done
                .cmp(&b.done)
                .then(a.display_order.cmp(&b.display_order))
                .then(a.id.cmp(&b.id))
        });

        let limit = Self::normalize_limit(max_items);
        let total_count = todos.len();
        let pending_count = todos.iter().filter(|item| !item.done).count();
        let items = todos
            .into_iter()
            .take(limit)
            .map(|item| WidgetTodoItem {
                id: item.id,
                text: item.text,
                done: item.done,
                category_id: item.category_id,
                category_name: item
                    .category_id
                    .and_then(|category_id| category_map.get(&category_id).cloned()),
                display_order: item.display_order,
                reminder_at: item.reminder_at,
                updated_at: item.updated_at,
            })
            .collect();

        Ok(WidgetSnapshot {
            generated_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            total_count,
            pending_count,
            items,
        })
    }

    pub fn refresh_cache(
        conn: &Connection,
        app: &AppHandle,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, String> {
        let snapshot = Self::get_snapshot(conn, max_items).map_err(|e| e.to_string())?;
        let cache_path = Self::resolve_cache_path(conn, app)?;

        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let snapshot_json = serde_json::to_string(&snapshot).map_err(|e| e.to_string())?;
        fs::write(&cache_path, snapshot_json).map_err(|e| e.to_string())?;

        Ok(snapshot)
    }

    pub fn set_cache_path(conn: &Connection, path: &str) -> Result<(), rusqlite::Error> {
        SettingsRepository::set(conn, WIDGET_CACHE_PATH_KEY, path)
    }

    pub fn set_app_group_id(conn: &Connection, app_group_id: &str) -> Result<(), rusqlite::Error> {
        SettingsRepository::set(conn, WIDGET_APP_GROUP_ID_KEY, app_group_id)
    }

    pub fn get_cache_path(conn: &Connection, app: &AppHandle) -> Result<String, String> {
        Ok(Self::resolve_cache_path(conn, app)?
            .as_os_str()
            .to_string_lossy()
            .to_string())
    }

    pub fn get_app_group_id(conn: &Connection) -> Result<String, String> {
        let app_group_id = SettingsRepository::get(conn, WIDGET_APP_GROUP_ID_KEY)
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| DEFAULT_WIDGET_APP_GROUP_ID.to_string());

        Ok(app_group_id.trim().to_string())
    }

    pub fn toggle_item_and_refresh(
        conn: &Connection,
        app: &AppHandle,
        id: i64,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, String> {
        crate::service::RepeatService::toggle_with_repeat(conn, id).map_err(|e| e.to_string())?;
        Self::refresh_cache(conn, app, max_items)
    }

    fn resolve_cache_path(conn: &Connection, app: &AppHandle) -> Result<PathBuf, String> {
        let custom_path = SettingsRepository::get(conn, WIDGET_CACHE_PATH_KEY)
            .map_err(|e| e.to_string())?
            .unwrap_or_default();
        let custom_path = custom_path.trim();

        if !custom_path.is_empty() {
            return Ok(PathBuf::from(custom_path));
        }

        if let Some(app_group_path) = Self::resolve_app_group_cache_path(conn)? {
            return Ok(app_group_path);
        }

        let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        Ok(app_data_dir.join(DEFAULT_WIDGET_CACHE_FILE))
    }

    fn resolve_app_group_cache_path(conn: &Connection) -> Result<Option<PathBuf>, String> {
        let app_group_id = Self::get_app_group_id(conn)?;

        if app_group_id.is_empty() {
            return Ok(None);
        }

        #[cfg(target_os = "ios")]
        {
            return Ok(resolve_ios_app_group_cache_path(&app_group_id));
        }

        #[cfg(not(target_os = "ios"))]
        {
            let _ = app_group_id;
            Ok(None)
        }
    }

    fn normalize_limit(max_items: Option<usize>) -> usize {
        max_items
            .unwrap_or(DEFAULT_WIDGET_ITEM_LIMIT)
            .clamp(1, MAX_WIDGET_ITEM_LIMIT)
    }
}

#[cfg(target_os = "ios")]
fn resolve_ios_app_group_cache_path(app_group_id: &str) -> Option<PathBuf> {
    use objc2_foundation::{NSFileManager, NSString};

    let group_identifier = NSString::from_str(app_group_id);
    let file_manager = NSFileManager::defaultManager();
    let container_url =
        file_manager.containerURLForSecurityApplicationGroupIdentifier(&group_identifier)?;
    let container_path = container_url.path()?;
    Some(PathBuf::from(container_path.to_string()).join(DEFAULT_WIDGET_CACHE_FILE))
}
