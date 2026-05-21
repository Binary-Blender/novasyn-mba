// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub id: String,
    pub name: String,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub stage: String,
    pub category: Option<String>,
    pub market_niche: Option<String>,
    pub business_model: Option<String>,
    pub revenue_target: f64,
    pub launch_date: Option<String>,
    pub exit_date: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBusinessInput {
    pub name: String,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub stage: Option<String>,
    pub category: Option<String>,
    pub market_niche: Option<String>,
    pub business_model: Option<String>,
    pub revenue_target: Option<f64>,
    pub launch_date: Option<String>,
    pub exit_date: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBusinessInput {
    pub name: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub stage: Option<String>,
    pub category: Option<String>,
    pub market_niche: Option<String>,
    pub business_model: Option<String>,
    pub revenue_target: Option<f64>,
    pub launch_date: Option<String>,
    pub exit_date: Option<String>,
    pub notes: Option<String>,
}

impl Business {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            name: row.get("name").unwrap(),
            tagline: row.get("tagline").ok(),
            description: row.get("description").ok(),
            stage: row.get("stage").unwrap(),
            category: row.get("category").ok(),
            market_niche: row.get("market_niche").ok(),
            business_model: row.get("business_model").ok(),
            revenue_target: row.get("revenue_target").unwrap(),
            launch_date: row.get("launch_date").ok(),
            exit_date: row.get("exit_date").ok(),
            notes: row.get("notes").ok(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_businesses(db: tauri::State<'_, DbPool>) -> Result<Vec<Business>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM businesses ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(Business::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_business(db: tauri::State<'_, DbPool>, input: CreateBusinessInput) -> Result<Business, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO businesses (id, name, tagline, description, stage, category, market_niche, business_model, revenue_target, launch_date, exit_date, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.name,
            input.tagline,
            input.description,
            input.stage.unwrap_or("Idea".to_string()),
            input.category,
            input.market_niche,
            input.business_model,
            input.revenue_target.unwrap_or(0),
            input.launch_date,
            input.exit_date,
            input.notes,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_business(db, id)
}

#[tauri::command]
pub fn get_business(db: tauri::State<'_, DbPool>, id: String) -> Result<Business, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM businesses WHERE id = ?", [&id], |row| {
        Ok(Business::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_business(db: tauri::State<'_, DbPool>, id: String, input: UpdateBusinessInput) -> Result<Business, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.name {
        sets.push("name = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.tagline {
        sets.push("tagline = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.description {
        sets.push("description = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.stage {
        sets.push("stage = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.category {
        sets.push("category = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.market_niche {
        sets.push("market_niche = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.business_model {
        sets.push("business_model = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.revenue_target {
        sets.push("revenue_target = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.launch_date {
        sets.push("launch_date = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.exit_date {
        sets.push("exit_date = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.notes {
        sets.push("notes = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE businesses SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_business(db, id)
}

#[tauri::command]
pub fn delete_business(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM businesses WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
