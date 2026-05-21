// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentPipelineItem {
    pub id: String,
    pub business_id: String,
    pub idea: String,
    pub status: String,
    pub platform: Option<String>,
    pub published_at: Option<String>,
    pub inbound_leads_generated: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContentPipelineItemInput {
    pub business_id: String,
    pub idea: String,
    pub status: Option<String>,
    pub platform: Option<String>,
    pub published_at: Option<String>,
    pub inbound_leads_generated: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContentPipelineItemInput {
    pub business_id: Option<String>,
    pub idea: Option<String>,
    pub status: Option<String>,
    pub platform: Option<String>,
    pub published_at: Option<String>,
    pub inbound_leads_generated: Option<i64>,
}

impl ContentPipelineItem {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            idea: row.get("idea").unwrap(),
            status: row.get("status").unwrap(),
            platform: row.get("platform").ok(),
            published_at: row.get("published_at").ok(),
            inbound_leads_generated: row.get("inbound_leads_generated").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_content_pipeline_items(db: tauri::State<'_, DbPool>) -> Result<Vec<ContentPipelineItem>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM content_pipeline_items ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(ContentPipelineItem::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_content_pipeline_item(db: tauri::State<'_, DbPool>, input: CreateContentPipelineItemInput) -> Result<ContentPipelineItem, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO content_pipeline_items (id, business_id, idea, status, platform, published_at, inbound_leads_generated, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.idea,
            input.status.unwrap_or("idea".to_string()),
            input.platform,
            input.published_at,
            input.inbound_leads_generated.unwrap_or(0),
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_content_pipeline_item(db, id)
}

#[tauri::command]
pub fn get_content_pipeline_item(db: tauri::State<'_, DbPool>, id: String) -> Result<ContentPipelineItem, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM content_pipeline_items WHERE id = ?", [&id], |row| {
        Ok(ContentPipelineItem::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_content_pipeline_item(db: tauri::State<'_, DbPool>, id: String, input: UpdateContentPipelineItemInput) -> Result<ContentPipelineItem, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.idea {
        sets.push("idea = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.status {
        sets.push("status = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.platform {
        sets.push("platform = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.published_at {
        sets.push("published_at = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.inbound_leads_generated {
        sets.push("inbound_leads_generated = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE content_pipeline_items SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_content_pipeline_item(db, id)
}

#[tauri::command]
pub fn delete_content_pipeline_item(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM content_pipeline_items WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
