use rusqlite::Connection;

use crate::models::graph::{GraphData, GraphEdge, GraphNode};

pub struct GraphRepository;

impl GraphRepository {
    pub fn get_graph_data(conn: &Connection) -> Result<GraphData, rusqlite::Error> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Get all non-deleted categories
        let mut cat_stmt = conn.prepare(
            "SELECT id, name FROM categories WHERE sync_status != 'deleted' OR sync_status IS NULL",
        )?;
        let cat_nodes = cat_stmt.query_map([], |row| {
            Ok(GraphNode {
                id: row.get(0)?,
                node_type: "category".to_string(),
                label: row.get(1)?,
                category_id: None,
                done: None,
            })
        })?;
        for node in cat_nodes {
            nodes.push(node?);
        }

        // Get all non-deleted tags
        let mut tag_stmt = conn.prepare(
            "SELECT id, name FROM tags WHERE sync_status != 'deleted' OR sync_status IS NULL",
        )?;
        let tag_nodes = tag_stmt.query_map([], |row| {
            Ok(GraphNode {
                id: row.get(0)?,
                node_type: "tag".to_string(),
                label: row.get(1)?,
                category_id: None,
                done: None,
            })
        })?;
        for node in tag_nodes {
            nodes.push(node?);
        }

        // Get all non-deleted items
        let mut item_stmt = conn.prepare(
            "SELECT id, text, category_id, done FROM todos WHERE sync_status != 'deleted' OR sync_status IS NULL",
        )?;
        let item_nodes = item_stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let category_id: Option<i64> = row.get(2)?;
            Ok((
                GraphNode {
                    id,
                    node_type: "item".to_string(),
                    label: row.get(1)?,
                    category_id,
                    done: row.get(3)?,
                },
                category_id,
            ))
        })?;
        for result in item_nodes {
            let (node, category_id) = result?;
            // Add item→category edge
            if let Some(cat_id) = category_id {
                edges.push(GraphEdge {
                    source: format!("item-{}", node.id),
                    target: format!("category-{}", cat_id),
                });
            }
            nodes.push(node);
        }

        // Get all active todo_tag associations
        let mut edge_stmt = conn.prepare(
            "SELECT todo_id, tag_id FROM todo_tags WHERE sync_status != 'deleted' OR sync_status IS NULL",
        )?;
        let tag_edges = edge_stmt.query_map([], |row| {
            let todo_id: i64 = row.get(0)?;
            let tag_id: i64 = row.get(1)?;
            Ok(GraphEdge {
                source: format!("item-{}", todo_id),
                target: format!("tag-{}", tag_id),
            })
        })?;
        for edge in tag_edges {
            edges.push(edge?);
        }

        Ok(GraphData { nodes, edges })
    }
}
