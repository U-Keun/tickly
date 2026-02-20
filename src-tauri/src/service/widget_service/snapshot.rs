use std::collections::HashMap;

use super::*;

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
                let tags = TodoTagRepository::get_tags_for_item(conn, todo.id)?
                    .into_iter()
                    .map(|tag| tag.name)
                    .collect();
                pending_item_ids_map
                    .entry(todo.category_id)
                    .or_default()
                    .push(todo.id);
                pending_items_map.entry(todo.category_id).or_default().push(
                    WidgetCategoryPendingItem {
                        id: todo.id,
                        text: todo.text.clone(),
                        display_order: todo.display_order,
                        tags,
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

        let theme = Self::resolve_widget_theme(conn);

        Ok(WidgetSnapshot {
            generated_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            total_count,
            pending_count,
            items,
            categories,
            theme,
        })
    }
}
