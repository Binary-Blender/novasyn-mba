// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioSynergy {
    pub id: String,
    pub business_a_id: i64,
    pub business_b_id: i64,
    pub synergy_type: String,
    pub description: Option<String>,
    pub potential_value: Option<String>,
    pub status: String,
    pub business_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePortfolioSynergyInput {
    pub business_a_id: i64,
    pub business_b_id: i64,
    pub synergy_type: Option<String>,
    pub description: Option<String>,
    pub potential_value: Option<String>,
    pub status: Option<String>,
    pub business_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePortfolioSynergyInput {
    pub business_a_id: Option<i64>,
    pub business_b_id: Option<i64>,
    pub synergy_type: Option<String>,
    pub description: Option<String>,
    pub potential_value: Option<String>,
    pub status: Option<String>,
}

impl PortfolioSynergy {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_a_id: row.get("business_a_id").unwrap(),
            business_b_id: row.get("business_b_id").unwrap(),
            synergy_type: row.get("synergy_type").unwrap(),
            description: row.get("description").ok(),
            potential_value: row.get("potential_value").ok(),
            status: row.get("status").unwrap(),
            business_id: row.get("business_id").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_portfolio_synergies(db: tauri::State<'_, DbPool>) -> Result<Vec<PortfolioSynergy>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM portfolio_synergies ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(PortfolioSynergy::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_portfolio_synergy(db: tauri::State<'_, DbPool>, input: CreatePortfolioSynergyInput) -> Result<PortfolioSynergy, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO portfolio_synergies (id, business_a_id, business_b_id, synergy_type, description, potential_value, status, business_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_a_id,
            input.business_b_id,
            input.synergy_type.unwrap_or("audience".to_string()),
            input.description,
            input.potential_value,
            input.status.unwrap_or("identified".to_string()),
            input.business_id,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_portfolio_synergy(db, id)
}

#[tauri::command]
pub fn get_portfolio_synergy(db: tauri::State<'_, DbPool>, id: String) -> Result<PortfolioSynergy, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM portfolio_synergies WHERE id = ?", [&id], |row| {
        Ok(PortfolioSynergy::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_portfolio_synergy(db: tauri::State<'_, DbPool>, id: String, input: UpdatePortfolioSynergyInput) -> Result<PortfolioSynergy, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_a_id {
        sets.push("business_a_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.business_b_id {
        sets.push("business_b_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.synergy_type {
        sets.push("synergy_type = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.description {
        sets.push("description = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.potential_value {
        sets.push("potential_value = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.status {
        sets.push("status = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE portfolio_synergies SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_portfolio_synergy(db, id)
}

#[tauri::command]
pub fn delete_portfolio_synergy(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM portfolio_synergies WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
