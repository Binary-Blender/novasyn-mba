// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketingAsset {
    pub id: String,
    pub business_id: i64,
    pub asset_type: String,
    pub title: String,
    pub url: Option<String>,
    pub platform: Option<String>,
    pub published_at: Option<String>,
    pub authority_stage_impact: Option<String>,
    pub business_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarketingAssetInput {
    pub business_id: i64,
    pub asset_type: Option<String>,
    pub title: String,
    pub url: Option<String>,
    pub platform: Option<String>,
    pub published_at: Option<String>,
    pub authority_stage_impact: Option<String>,
    pub business_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMarketingAssetInput {
    pub business_id: Option<i64>,
    pub asset_type: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub platform: Option<String>,
    pub published_at: Option<String>,
    pub authority_stage_impact: Option<String>,
}

impl MarketingAsset {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            asset_type: row.get("asset_type").unwrap(),
            title: row.get("title").unwrap(),
            url: row.get("url").ok(),
            platform: row.get("platform").ok(),
            published_at: row.get("published_at").ok(),
            authority_stage_impact: row.get("authority_stage_impact").ok(),
            business_id: row.get("business_id").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_marketing_assets(db: tauri::State<'_, DbPool>) -> Result<Vec<MarketingAsset>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM marketing_assets ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(MarketingAsset::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_marketing_asset(db: tauri::State<'_, DbPool>, input: CreateMarketingAssetInput) -> Result<MarketingAsset, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO marketing_assets (id, business_id, asset_type, title, url, platform, published_at, authority_stage_impact, business_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.asset_type.unwrap_or("content".to_string()),
            input.title,
            input.url,
            input.platform,
            input.published_at,
            input.authority_stage_impact,
            input.business_id,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_marketing_asset(db, id)
}

#[tauri::command]
pub fn get_marketing_asset(db: tauri::State<'_, DbPool>, id: String) -> Result<MarketingAsset, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM marketing_assets WHERE id = ?", [&id], |row| {
        Ok(MarketingAsset::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_marketing_asset(db: tauri::State<'_, DbPool>, id: String, input: UpdateMarketingAssetInput) -> Result<MarketingAsset, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.asset_type {
        sets.push("asset_type = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.title {
        sets.push("title = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.url {
        sets.push("url = ?".to_string());
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
    if let Some(val) = input.authority_stage_impact {
        sets.push("authority_stage_impact = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE marketing_assets SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_marketing_asset(db, id)
}

#[tauri::command]
pub fn delete_marketing_asset(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM marketing_assets WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
