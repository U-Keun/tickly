use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{Category, SyncResult, SyncStatus, TodoItem};
use crate::repository::{CategoryRepository, CompletionLogRepository, SyncRepository, TagRepository, TodoRepository, TodoTagRepository};

use super::supabase_client::{RemoteCategory, RemoteCompletionLog, RemoteTag, RemoteTodo, RemoteTodoTag, SupabaseClient};

pub struct SyncService;

/// Data structures to pass between sync phases (to avoid holding connection across async)
#[derive(Debug, Clone)]
struct PendingCategorySync {
    id: i64,
    sync_id: Option<String>,
    name: String,
    display_order: i64,
    created_at: Option<String>,
    updated_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
struct PendingTodoSync {
    id: i64,
    sync_id: Option<String>,
    category_sync_id: Option<String>,
    text: String,
    done: bool,
    display_order: i64,
    memo: Option<String>,
    repeat_type: String,
    repeat_detail: Option<String>,
    next_due_at: Option<String>,
    last_completed_at: Option<String>,
    track_streak: bool,
    reminder_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
struct PendingTagSync {
    id: i64,
    sync_id: Option<String>,
    name: String,
    created_at: Option<String>,
    updated_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PendingTodoTagSync {
    todo_id: i64,
    tag_id: i64,
    sync_id: Option<String>,
    todo_sync_id: Option<String>,
    tag_sync_id: Option<String>,
    created_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LocalCompletionLogSync {
    item_id: i64,
    todo_sync_id: String,
    completed_on: String,
    completed_count: i32,
}

impl SyncService {
    /// Blocking version that uses tokio runtime internally
    pub fn sync_all_blocking(
        conn: &Connection,
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
    ) -> Result<SyncResult, String> {
        // Create a new runtime for blocking execution
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        // Collect pending categories and generate sync_ids for those without
        let mut pending_categories = Self::collect_pending_categories(conn)?;

        // Pre-generate sync_ids for categories that don't have them
        // This allows us to use these sync_ids when collecting todos
        let mut cat_id_to_sync_id: std::collections::HashMap<i64, String> = std::collections::HashMap::new();
        for cat in &mut pending_categories {
            if cat.sync_id.is_none() {
                let new_sync_id = Uuid::new_v4().to_string();
                cat.sync_id = Some(new_sync_id.clone());
                cat_id_to_sync_id.insert(cat.id, new_sync_id);
            } else {
                cat_id_to_sync_id.insert(cat.id, cat.sync_id.clone().unwrap());
            }
        }

        // Also include existing synced categories in the map (including deleted ones for proper mapping)
        let all_categories = CategoryRepository::get_all_including_deleted(conn).map_err(|e| e.to_string())?;
        for cat in &all_categories {
            if let Some(sync_id) = &cat.sync_id {
                cat_id_to_sync_id.insert(cat.id, sync_id.clone());
            }
        }

        // Now collect pending todos with correct category sync_ids
        let pending_todos = Self::collect_pending_todos_with_map(conn, &cat_id_to_sync_id)?;

        // Build todo id to sync_id map for completion logs
        let all_todos = TodoRepository::get_all(conn).map_err(|e| e.to_string())?;
        let mut todo_id_to_sync_id: HashMap<i64, String> = HashMap::new();
        for todo in &all_todos {
            if let Some(sync_id) = &todo.sync_id {
                todo_id_to_sync_id.insert(todo.id, sync_id.clone());
            }
        }
        // Also include pending todos' pre-generated sync_ids
        for todo in &pending_todos {
            if let Some(sync_id) = &todo.sync_id {
                todo_id_to_sync_id.insert(todo.id, sync_id.clone());
            }
        }

        // Collect completion logs
        let local_completion_logs = Self::collect_completion_logs(conn, &todo_id_to_sync_id)?;

        // Collect pending tags
        let pending_tags = Self::collect_pending_tags(conn)?;

        // Build tag id to sync_id map
        let all_tags = TagRepository::get_all(conn).map_err(|e| e.to_string())?;
        let mut tag_id_to_sync_id: HashMap<i64, String> = HashMap::new();
        for tag in &all_tags {
            if let Some(sync_id) = &tag.sync_id {
                tag_id_to_sync_id.insert(tag.id, sync_id.clone());
            }
        }
        for tag in &pending_tags {
            if let Some(sync_id) = &tag.sync_id {
                tag_id_to_sync_id.insert(tag.id, sync_id.clone());
            }
        }

        // Collect pending todo_tags
        let pending_todo_tags = Self::collect_pending_todo_tags(conn, &todo_id_to_sync_id, &tag_id_to_sync_id)?;

        // Run async operations
        let result = rt.block_on(async {
            let mut result = SyncResult::default();

            // Push categories
            let pushed_cats =
                Self::push_categories_async(client, access_token, user_id, &pending_categories)
                    .await?;

            // Push todos
            let pushed_todos =
                Self::push_todos_async(client, access_token, user_id, &pending_todos).await?;

            // Push completion logs
            let pushed_logs =
                Self::push_completion_logs_async(client, access_token, user_id, &local_completion_logs)
                    .await?;

            // Push tags
            let pushed_tags =
                Self::push_tags_async(client, access_token, user_id, &pending_tags).await?;

            // Push todo_tags
            let pushed_todo_tags =
                Self::push_todo_tags_async(client, access_token, user_id, &pending_todo_tags).await?;

            result.pushed = pushed_cats.len() + pushed_todos.len() + pushed_logs + pushed_tags.len() + pushed_todo_tags;

            // Pull remote data
            let remote_categories = client.fetch_categories(access_token).await?;
            let remote_todos = client.fetch_todos(access_token).await?;
            let remote_completion_logs = client.fetch_all_completion_logs(access_token).await?;
            let remote_tags = client.fetch_tags(access_token).await.unwrap_or_default();
            let remote_todo_tags = client.fetch_todo_tags(access_token).await.unwrap_or_default();

            Ok::<_, String>((result, pushed_cats, pushed_todos, pushed_tags, remote_categories, remote_todos, remote_completion_logs, remote_tags, remote_todo_tags))
        })?;

        let (mut sync_result, pushed_cats, pushed_todos, pushed_tags, remote_categories, remote_todos, remote_completion_logs, remote_tags, remote_todo_tags) = result;

        // Update local database with push results
        for (local_id, sync_id) in pushed_cats {
            if let Some(cat) = pending_categories.iter().find(|c| c.id == local_id) {
                if cat.sync_id.is_none() {
                    CategoryRepository::update_sync_id(conn, local_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if cat.sync_status == SyncStatus::Deleted {
                    CategoryRepository::delete(conn, local_id).map_err(|e| e.to_string())?;
                } else {
                    CategoryRepository::mark_synced(conn, local_id).map_err(|e| e.to_string())?;
                }
            }
        }

        for (local_id, sync_id) in pushed_todos {
            if let Some(todo) = pending_todos.iter().find(|t| t.id == local_id) {
                if todo.sync_id.is_none() {
                    TodoRepository::update_sync_id(conn, local_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if todo.sync_status == SyncStatus::Deleted {
                    TodoRepository::delete(conn, local_id).map_err(|e| e.to_string())?;
                } else {
                    TodoRepository::mark_synced(conn, local_id).map_err(|e| e.to_string())?;
                }
            }
        }

        // Update local tags with push results
        for (local_id, sync_id) in pushed_tags {
            if let Some(tag) = pending_tags.iter().find(|t| t.id == local_id) {
                if tag.sync_id.is_none() {
                    TagRepository::update_sync_id(conn, local_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if tag.sync_status == SyncStatus::Deleted {
                    TagRepository::delete(conn, local_id).map_err(|e| e.to_string())?;
                } else {
                    TagRepository::mark_synced(conn, local_id).map_err(|e| e.to_string())?;
                }
            }
        }

        // Re-fetch local data AFTER push to get updated sync_ids
        let updated_local_categories = CategoryRepository::get_all(conn).map_err(|e| e.to_string())?;
        let updated_local_todos = TodoRepository::get_all(conn).map_err(|e| e.to_string())?;

        // Pull changes - use updated local data
        let pulled = Self::apply_remote_changes(
            conn,
            &updated_local_categories,
            &updated_local_todos,
            remote_categories,
            remote_todos,
        )?;

        // Apply remote completion logs
        let pulled_logs = Self::apply_remote_completion_logs(conn, &updated_local_todos, remote_completion_logs)?;

        // Apply remote tags
        let pulled_tags = Self::apply_remote_tags(conn, remote_tags)?;

        // Apply remote todo_tags
        let pulled_todo_tags = Self::apply_remote_todo_tags(conn, remote_todo_tags)?;

        sync_result.pulled = pulled + pulled_logs + pulled_tags + pulled_todo_tags;

        // Update last synced timestamp
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        SyncRepository::set_last_synced_at(conn, &now).map_err(|e| e.to_string())?;
        sync_result.last_synced_at = Some(now);

        Ok(sync_result)
    }

    fn collect_pending_categories(conn: &Connection) -> Result<Vec<PendingCategorySync>, String> {
        let categories =
            CategoryRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;
        Ok(categories
            .into_iter()
            .map(|c| PendingCategorySync {
                id: c.id,
                sync_id: c.sync_id,
                name: c.name,
                display_order: c.display_order,
                created_at: c.created_at,
                updated_at: c.updated_at,
                sync_status: c.sync_status,
            })
            .collect())
    }

    #[allow(dead_code)]
    fn collect_pending_todos(conn: &Connection) -> Result<Vec<PendingTodoSync>, String> {
        let cat_sync_map: HashMap<i64, String> = CategoryRepository::get_all(conn)
            .map_err(|e| e.to_string())?
            .iter()
            .filter_map(|c| c.sync_id.as_ref().map(|s| (c.id, s.clone())))
            .collect();

        Self::collect_pending_todos_with_map(conn, &cat_sync_map)
    }

    fn collect_pending_todos_with_map(
        conn: &Connection,
        cat_sync_map: &HashMap<i64, String>,
    ) -> Result<Vec<PendingTodoSync>, String> {
        let todos = TodoRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;

        Ok(todos
            .into_iter()
            .map(|t| PendingTodoSync {
                id: t.id,
                sync_id: t.sync_id,
                category_sync_id: t.category_id.and_then(|cid| cat_sync_map.get(&cid).cloned()),
                text: t.text,
                done: t.done,
                display_order: t.display_order,
                memo: t.memo,
                repeat_type: t.repeat_type.to_str().to_string(),
                repeat_detail: t.repeat_detail,
                next_due_at: t.next_due_at,
                last_completed_at: t.last_completed_at,
                track_streak: t.track_streak,
                reminder_at: t.reminder_at,
                created_at: t.created_at,
                updated_at: t.updated_at,
                sync_status: t.sync_status,
            })
            .collect())
    }

    async fn push_categories_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        categories: &[PendingCategorySync],
    ) -> Result<Vec<(i64, String)>, String> {
        let mut results = Vec::new();

        for cat in categories {
            match cat.sync_status {
                SyncStatus::Pending => {
                    let sync_id = cat
                        .sync_id
                        .clone()
                        .unwrap_or_else(|| Uuid::new_v4().to_string());

                    let remote = RemoteCategory {
                        id: sync_id.clone(),
                        user_id: user_id.to_string(),
                        name: cat.name.clone(),
                        display_order: cat.display_order as i32,
                        created_at: cat.created_at.clone().unwrap_or_else(|| {
                            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                        }),
                        updated_at: cat.updated_at.clone().unwrap_or_else(|| {
                            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                        }),
                    };

                    client.upsert_category(access_token, &remote).await?;
                    results.push((cat.id, sync_id));
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &cat.sync_id {
                        client.delete_category(access_token, sync_id).await?;
                        results.push((cat.id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    async fn push_todos_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        todos: &[PendingTodoSync],
    ) -> Result<Vec<(i64, String)>, String> {
        let mut results = Vec::new();

        for todo in todos {
            match todo.sync_status {
                SyncStatus::Pending => {
                    let sync_id = todo
                        .sync_id
                        .clone()
                        .unwrap_or_else(|| Uuid::new_v4().to_string());

                    let remote = RemoteTodo {
                        id: sync_id.clone(),
                        user_id: user_id.to_string(),
                        category_id: todo.category_sync_id.clone(),
                        text: todo.text.clone(),
                        done: todo.done,
                        display_order: todo.display_order as i32,
                        memo: todo.memo.clone(),
                        repeat_type: todo.repeat_type.clone(),
                        repeat_detail: todo.repeat_detail.clone(),
                        next_due_at: todo.next_due_at.clone(),
                        last_completed_at: todo.last_completed_at.clone(),
                        track_streak: todo.track_streak,
                        reminder_at: todo.reminder_at.clone(),
                        created_at: todo.created_at.clone().unwrap_or_else(|| {
                            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                        }),
                        updated_at: todo.updated_at.clone().unwrap_or_else(|| {
                            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                        }),
                    };

                    client.upsert_todo(access_token, &remote).await?;
                    results.push((todo.id, sync_id));
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &todo.sync_id {
                        client.delete_todo(access_token, sync_id).await?;
                        results.push((todo.id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    fn apply_remote_changes(
        conn: &Connection,
        local_categories: &[Category],
        local_todos: &[TodoItem],
        remote_categories: Vec<RemoteCategory>,
        remote_todos: Vec<RemoteTodo>,
    ) -> Result<usize, String> {
        let mut count = 0;

        // Build category sync map
        let category_sync_map: HashMap<String, i64> = local_categories
            .iter()
            .filter_map(|c| c.sync_id.as_ref().map(|s| (s.clone(), c.id)))
            .collect();

        // Process remote categories
        for remote in &remote_categories {
            if let Some(&local_id) = category_sync_map.get(&remote.id) {
                if let Some(local) = local_categories.iter().find(|c| c.id == local_id) {
                    if Self::is_remote_newer(&local.updated_at, &remote.updated_at) {
                        Self::update_local_category(conn, local, remote)?;
                        count += 1;
                    }
                }
            } else {
                Self::insert_category_from_remote(conn, remote)?;
                count += 1;
            }
        }

        // Rebuild category map after inserts
        let updated_categories =
            CategoryRepository::get_all(conn).map_err(|e| e.to_string())?;
        let category_sync_map: HashMap<String, i64> = updated_categories
            .iter()
            .filter_map(|c| c.sync_id.as_ref().map(|s| (s.clone(), c.id)))
            .collect();

        // Build todo sync map
        let todo_sync_map: HashMap<String, i64> = local_todos
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        // Process remote todos
        for remote in &remote_todos {
            let local_category_id = remote
                .category_id
                .as_ref()
                .and_then(|sync_id| category_sync_map.get(sync_id).copied());

            if let Some(&local_id) = todo_sync_map.get(&remote.id) {
                if let Some(local) = local_todos.iter().find(|t| t.id == local_id) {
                    if Self::is_remote_newer(&local.updated_at, &remote.updated_at) {
                        Self::update_local_todo(conn, local, remote, local_category_id)?;
                        count += 1;
                    }
                }
            } else {
                Self::insert_todo_from_remote(conn, remote, local_category_id)?;
                count += 1;
            }
        }

        Ok(count)
    }

    fn is_remote_newer(local_updated: &Option<String>, remote_updated: &str) -> bool {
        match local_updated {
            Some(local) => {
                let local_dt = chrono::DateTime::parse_from_rfc3339(local).ok();
                let remote_dt = chrono::DateTime::parse_from_rfc3339(remote_updated).ok();

                match (local_dt, remote_dt) {
                    (Some(l), Some(r)) => r > l,
                    _ => true,
                }
            }
            None => true,
        }
    }

    fn update_local_category(
        conn: &Connection,
        local: &Category,
        remote: &RemoteCategory,
    ) -> Result<(), String> {
        conn.execute(
            "UPDATE categories SET name = ?1, display_order = ?2, updated_at = ?3, sync_status = 'synced' WHERE id = ?4",
            rusqlite::params![remote.name, remote.display_order, remote.updated_at, local.id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn insert_category_from_remote(conn: &Connection, remote: &RemoteCategory) -> Result<(), String> {
        conn.execute(
            "INSERT INTO categories (name, display_order, sync_id, created_at, updated_at, sync_status)
             VALUES (?1, ?2, ?3, ?4, ?5, 'synced')
             ON CONFLICT(sync_id) DO UPDATE SET
                name = excluded.name,
                display_order = excluded.display_order,
                updated_at = excluded.updated_at,
                sync_status = 'synced'",
            rusqlite::params![
                remote.name,
                remote.display_order,
                remote.id,
                remote.created_at,
                remote.updated_at
            ],
        )
        .map_err(|e| format!("Failed to insert category: {}", e))?;
        Ok(())
    }

    fn update_local_todo(
        conn: &Connection,
        local: &TodoItem,
        remote: &RemoteTodo,
        local_category_id: Option<i64>,
    ) -> Result<(), String> {
        conn.execute(
            "UPDATE todos SET text = ?1, done = ?2, category_id = ?3, display_order = ?4, memo = ?5,
             repeat_type = ?6, repeat_detail = ?7, next_due_at = ?8, last_completed_at = ?9,
             track_streak = ?10, reminder_at = ?11, updated_at = ?12, sync_status = 'synced' WHERE id = ?13",
            rusqlite::params![
                remote.text,
                remote.done,
                local_category_id,
                remote.display_order,
                remote.memo,
                remote.repeat_type,
                remote.repeat_detail,
                remote.next_due_at,
                remote.last_completed_at,
                remote.track_streak,
                remote.reminder_at,
                remote.updated_at,
                local.id
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn insert_todo_from_remote(
        conn: &Connection,
        remote: &RemoteTodo,
        local_category_id: Option<i64>,
    ) -> Result<(), String> {
        conn.execute(
            "INSERT INTO todos (text, done, category_id, display_order, memo, repeat_type, repeat_detail,
             next_due_at, last_completed_at, track_streak, reminder_at, sync_id, created_at, updated_at, sync_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, 'synced')
             ON CONFLICT(sync_id) DO UPDATE SET
                text = excluded.text,
                done = excluded.done,
                category_id = excluded.category_id,
                display_order = excluded.display_order,
                memo = excluded.memo,
                repeat_type = excluded.repeat_type,
                repeat_detail = excluded.repeat_detail,
                next_due_at = excluded.next_due_at,
                last_completed_at = excluded.last_completed_at,
                track_streak = excluded.track_streak,
                reminder_at = excluded.reminder_at,
                updated_at = excluded.updated_at,
                sync_status = 'synced'",
            rusqlite::params![
                remote.text,
                remote.done,
                local_category_id,
                remote.display_order,
                remote.memo,
                remote.repeat_type,
                remote.repeat_detail,
                remote.next_due_at,
                remote.last_completed_at,
                remote.track_streak,
                remote.reminder_at,
                remote.id,
                remote.created_at,
                remote.updated_at
            ],
        )
        .map_err(|e| format!("Failed to insert todo: {}", e))?;
        Ok(())
    }

    fn collect_pending_tags(conn: &Connection) -> Result<Vec<PendingTagSync>, String> {
        let tags = TagRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;
        Ok(tags
            .into_iter()
            .map(|t| PendingTagSync {
                id: t.id,
                sync_id: t.sync_id.clone().or_else(|| Some(Uuid::new_v4().to_string())),
                name: t.name,
                created_at: t.created_at,
                updated_at: t.updated_at,
                sync_status: t.sync_status,
            })
            .collect())
    }

    fn collect_pending_todo_tags(
        conn: &Connection,
        todo_id_to_sync_id: &HashMap<i64, String>,
        tag_id_to_sync_id: &HashMap<i64, String>,
    ) -> Result<Vec<PendingTodoTagSync>, String> {
        let todo_tags = TodoTagRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;
        Ok(todo_tags
            .into_iter()
            .filter_map(|tt| {
                let todo_sync_id = todo_id_to_sync_id.get(&tt.todo_id).cloned();
                let tag_sync_id = tag_id_to_sync_id.get(&tt.tag_id).cloned();
                // Skip if we can't resolve both sync_ids
                if todo_sync_id.is_none() || tag_sync_id.is_none() {
                    return None;
                }
                Some(PendingTodoTagSync {
                    todo_id: tt.todo_id,
                    tag_id: tt.tag_id,
                    sync_id: tt.sync_id.clone().or_else(|| Some(Uuid::new_v4().to_string())),
                    todo_sync_id,
                    tag_sync_id,
                    created_at: tt.created_at,
                    sync_status: tt.sync_status,
                })
            })
            .collect())
    }

    async fn push_tags_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        tags: &[PendingTagSync],
    ) -> Result<Vec<(i64, String)>, String> {
        let mut results = Vec::new();

        for tag in tags {
            let sync_id = tag.sync_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

            match tag.sync_status {
                SyncStatus::Pending => {
                    let remote = RemoteTag {
                        id: sync_id.clone(),
                        user_id: user_id.to_string(),
                        name: tag.name.clone(),
                        created_at: tag.created_at.clone().unwrap_or_else(|| {
                            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                        }),
                        updated_at: tag.updated_at.clone().unwrap_or_else(|| {
                            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                        }),
                    };
                    client.upsert_tag(access_token, &remote).await?;
                    results.push((tag.id, sync_id));
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &tag.sync_id {
                        client.delete_tag(access_token, sync_id).await?;
                        results.push((tag.id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    async fn push_todo_tags_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        todo_tags: &[PendingTodoTagSync],
    ) -> Result<usize, String> {
        let mut count = 0;

        for tt in todo_tags {
            let sync_id = tt.sync_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

            match tt.sync_status {
                SyncStatus::Pending => {
                    if let (Some(todo_sync_id), Some(tag_sync_id)) = (&tt.todo_sync_id, &tt.tag_sync_id) {
                        let remote = RemoteTodoTag {
                            id: sync_id,
                            user_id: user_id.to_string(),
                            todo_id: todo_sync_id.clone(),
                            tag_id: tag_sync_id.clone(),
                            created_at: tt.created_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                        };
                        client.upsert_todo_tag(access_token, &remote).await?;
                        count += 1;
                    }
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &tt.sync_id {
                        client.delete_todo_tag(access_token, sync_id).await?;
                        count += 1;
                    }
                }
                _ => {}
            }
        }

        Ok(count)
    }

    fn apply_remote_tags(
        conn: &Connection,
        remote_tags: Vec<RemoteTag>,
    ) -> Result<usize, String> {
        let mut count = 0;

        for remote in remote_tags {
            let existing = TagRepository::get_by_sync_id(conn, &remote.id).map_err(|e| e.to_string())?;
            if let Some(local) = existing {
                // Update if remote is newer
                if Self::is_remote_newer(&local.updated_at, &remote.updated_at) {
                    conn.execute(
                        "UPDATE tags SET name = ?1, updated_at = ?2, sync_status = 'synced' WHERE id = ?3",
                        rusqlite::params![remote.name, remote.updated_at, local.id],
                    ).map_err(|e| e.to_string())?;
                    count += 1;
                }
            } else {
                // Check if tag with same name exists (merge)
                if let Some(local_by_name) = TagRepository::get_by_name(conn, &remote.name).map_err(|e| e.to_string())? {
                    // Assign sync_id to existing local tag
                    TagRepository::update_sync_id(conn, local_by_name.id, &remote.id).map_err(|e| e.to_string())?;
                } else {
                    // Insert new tag
                    conn.execute(
                        "INSERT INTO tags (name, sync_id, created_at, updated_at, sync_status) VALUES (?1, ?2, ?3, ?4, 'synced')",
                        rusqlite::params![remote.name, remote.id, remote.created_at, remote.updated_at],
                    ).map_err(|e| e.to_string())?;
                }
                count += 1;
            }
        }

        Ok(count)
    }

    fn apply_remote_todo_tags(
        conn: &Connection,
        remote_todo_tags: Vec<RemoteTodoTag>,
    ) -> Result<usize, String> {
        let mut count = 0;

        // Build sync_id to local_id maps
        let all_todos = TodoRepository::get_all(conn).map_err(|e| e.to_string())?;
        let todo_sync_to_local: HashMap<String, i64> = all_todos
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        let all_tags = TagRepository::get_all(conn).map_err(|e| e.to_string())?;
        let tag_sync_to_local: HashMap<String, i64> = all_tags
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        for remote in remote_todo_tags {
            let local_todo_id = todo_sync_to_local.get(&remote.todo_id).copied();
            let local_tag_id = tag_sync_to_local.get(&remote.tag_id).copied();

            if let (Some(todo_id), Some(tag_id)) = (local_todo_id, local_tag_id) {
                // Check if this todo_tag already exists
                let existing = TodoTagRepository::get_by_sync_id(conn, &remote.id).map_err(|e| e.to_string())?;
                if existing.is_none() {
                    // Insert and set sync_id
                    TodoTagRepository::add_tag(conn, todo_id, tag_id).map_err(|e| e.to_string())?;
                    TodoTagRepository::update_sync_id(conn, todo_id, tag_id, &remote.id).map_err(|e| e.to_string())?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub fn get_pending_count(conn: &Connection) -> Result<usize, String> {
        SyncRepository::get_pending_count(conn).map_err(|e| e.to_string())
    }

    pub fn get_last_synced_at(conn: &Connection) -> Result<Option<String>, String> {
        SyncRepository::get_last_synced_at(conn).map_err(|e| e.to_string())
    }

    pub fn is_sync_enabled(conn: &Connection) -> Result<bool, String> {
        SyncRepository::is_sync_enabled(conn).map_err(|e| e.to_string())
    }

    pub fn set_sync_enabled(conn: &Connection, enabled: bool) -> Result<(), String> {
        SyncRepository::set_sync_enabled(conn, enabled).map_err(|e| e.to_string())
    }

    fn collect_completion_logs(
        conn: &Connection,
        todo_id_to_sync_id: &HashMap<i64, String>,
    ) -> Result<Vec<LocalCompletionLogSync>, String> {
        let logs = CompletionLogRepository::get_all(conn).map_err(|e| e.to_string())?;

        Ok(logs
            .into_iter()
            .filter_map(|log| {
                todo_id_to_sync_id.get(&log.item_id).map(|sync_id| LocalCompletionLogSync {
                    item_id: log.item_id,
                    todo_sync_id: sync_id.clone(),
                    completed_on: log.completed_on,
                    completed_count: log.completed_count,
                })
            })
            .collect())
    }

    async fn push_completion_logs_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        logs: &[LocalCompletionLogSync],
    ) -> Result<usize, String> {
        let mut count = 0;

        for log in logs {
            // Generate a deterministic ID based on todo_id + date
            let log_id = format!("{}_{}", log.todo_sync_id, log.completed_on);

            let remote = RemoteCompletionLog {
                id: log_id,
                user_id: user_id.to_string(),
                todo_id: log.todo_sync_id.clone(),
                completed_on: log.completed_on.clone(),
                completed_count: log.completed_count as i32,
            };

            client.upsert_completion_log(access_token, &remote).await?;
            count += 1;
        }

        Ok(count)
    }

    fn apply_remote_completion_logs(
        conn: &Connection,
        local_todos: &[TodoItem],
        remote_logs: Vec<RemoteCompletionLog>,
    ) -> Result<usize, String> {
        let mut count = 0;

        // Build sync_id to local_id map
        let sync_id_to_local_id: HashMap<String, i64> = local_todos
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        for remote in remote_logs {
            if let Some(&local_id) = sync_id_to_local_id.get(&remote.todo_id) {
                CompletionLogRepository::upsert(
                    conn,
                    local_id,
                    &remote.completed_on,
                    remote.completed_count,
                )
                .map_err(|e| e.to_string())?;
                count += 1;
            }
        }

        Ok(count)
    }
}
