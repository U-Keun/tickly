use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::models::{
    WidgetCategoryPendingItem, WidgetCategorySummary, WidgetSnapshot, WidgetTodoItem,
};
use crate::repository::{CategoryRepository, SettingsRepository, TodoRepository};

pub struct WidgetService;

const WIDGET_CACHE_PATH_KEY: &str = "widget_cache_path";
const WIDGET_APP_GROUP_ID_KEY: &str = "widget_app_group_id";
const DEFAULT_WIDGET_APP_GROUP_ID: &str = "group.com.u-keunsong.tickly";
const DEFAULT_WIDGET_CACHE_FILE: &str = "widget-cache.json";
const DEFAULT_WIDGET_ACTION_FILE: &str = "widget-actions.json";
const DEFAULT_WIDGET_ITEM_LIMIT: usize = 20;
const MAX_WIDGET_ITEM_LIMIT: usize = 100;

#[derive(Debug, Deserialize)]
struct WidgetToggleAction {
    #[serde(alias = "itemId")]
    item_id: i64,
}

impl WidgetService {
    pub fn get_snapshot(
        conn: &Connection,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, rusqlite::Error> {
        let mut todos = TodoRepository::get_all(conn)?;
        let categories = CategoryRepository::get_all(conn)?;
        let category_name_map: HashMap<i64, String> = categories
            .iter()
            .map(|cat| (cat.id, cat.name.clone()))
            .collect();
        let category_order_map: HashMap<i64, i64> = categories
            .iter()
            .map(|cat| (cat.id, cat.display_order))
            .collect();
        let mut category_counts: HashMap<Option<i64>, (usize, usize)> = HashMap::new();

        for todo in &todos {
            let entry = category_counts.entry(todo.category_id).or_insert((0, 0));
            entry.0 += 1;
            if !todo.done {
                entry.1 += 1;
            }
        }

        todos.sort_by(|a, b| {
            a.done
                .cmp(&b.done)
                .then(a.display_order.cmp(&b.display_order))
                .then(a.id.cmp(&b.id))
        });

        let mut pending_item_ids_map: HashMap<Option<i64>, Vec<i64>> = HashMap::new();
        let mut pending_items_map: HashMap<Option<i64>, Vec<WidgetCategoryPendingItem>> =
            HashMap::new();
        for todo in &todos {
            if !todo.done {
                pending_item_ids_map
                    .entry(todo.category_id)
                    .or_default()
                    .push(todo.id);
                pending_items_map.entry(todo.category_id).or_default().push(
                    WidgetCategoryPendingItem {
                        id: todo.id,
                        text: todo.text.clone(),
                        display_order: todo.display_order,
                    },
                );
            }
        }

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
                    .and_then(|category_id| category_name_map.get(&category_id).cloned()),
                display_order: item.display_order,
                reminder_at: item.reminder_at,
                updated_at: item.updated_at,
            })
            .collect();
        let mut categories: Vec<WidgetCategorySummary> = category_counts
            .into_iter()
            .map(|(category_id, (total_count, pending_count))| {
                let category_name = category_id
                    .and_then(|id| category_name_map.get(&id).cloned())
                    .unwrap_or_else(|| "Uncategorized".to_string());
                let pending_item_ids = pending_item_ids_map
                    .get(&category_id)
                    .cloned()
                    .unwrap_or_default();
                let pending_items = pending_items_map
                    .get(&category_id)
                    .cloned()
                    .unwrap_or_default();

                WidgetCategorySummary {
                    category_id,
                    category_name,
                    total_count,
                    pending_count,
                    first_pending_item_id: pending_item_ids.first().copied(),
                    pending_item_ids,
                    pending_items,
                }
            })
            .collect();

        categories.sort_by(|a, b| {
            Self::category_sort_order(a.category_id, &category_order_map)
                .cmp(&Self::category_sort_order(
                    b.category_id,
                    &category_order_map,
                ))
                .then(a.category_name.cmp(&b.category_name))
        });

        Ok(WidgetSnapshot {
            generated_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            total_count,
            pending_count,
            items,
            categories,
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
        crate::service::RepeatService::complete_with_repeat(conn, id).map_err(|e| e.to_string())?;
        Self::refresh_cache(conn, app, max_items)
    }

    pub fn process_pending_actions(
        conn: &Connection,
        app: &AppHandle,
        max_items: Option<usize>,
    ) -> Result<usize, String> {
        let actions_path = Self::resolve_actions_path(conn, app)?;
        let actions = Self::read_pending_actions(&actions_path)?;

        if actions.is_empty() {
            return Ok(0);
        }

        let mut processed = 0usize;
        let mut seen_item_ids = HashSet::new();
        for action in actions {
            if !seen_item_ids.insert(action.item_id) {
                continue;
            }

            match crate::service::RepeatService::complete_with_repeat(conn, action.item_id) {
                Ok(Some(_)) => processed += 1,
                Ok(None) => {}
                Err(error) => {
                    log::error!(
                        "Failed to process widget action for item {}: {}",
                        action.item_id,
                        error
                    );
                }
            }
        }

        Self::clear_pending_actions(&actions_path)?;
        Self::refresh_cache(conn, app, max_items)?;
        Ok(processed)
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

    fn resolve_actions_path(conn: &Connection, app: &AppHandle) -> Result<PathBuf, String> {
        let cache_path = Self::resolve_cache_path(conn, app)?;
        if let Some(parent) = cache_path.parent() {
            return Ok(parent.join(DEFAULT_WIDGET_ACTION_FILE));
        }
        Ok(cache_path.with_file_name(DEFAULT_WIDGET_ACTION_FILE))
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

    fn category_sort_order(
        category_id: Option<i64>,
        category_order_map: &HashMap<i64, i64>,
    ) -> i64 {
        category_id
            .and_then(|id| category_order_map.get(&id).copied())
            .unwrap_or(i64::MAX)
    }

    fn read_pending_actions(actions_path: &PathBuf) -> Result<Vec<WidgetToggleAction>, String> {
        if !actions_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(actions_path).map_err(|e| e.to_string())?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        match serde_json::from_str::<Vec<WidgetToggleAction>>(&content) {
            Ok(actions) => Ok(actions),
            Err(error) => {
                log::error!("Failed to decode widget action queue: {}", error);
                let _ = fs::write(actions_path, "[]");
                Ok(Vec::new())
            }
        }
    }

    fn clear_pending_actions(actions_path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = actions_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(actions_path, "[]").map_err(|e| e.to_string())
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
