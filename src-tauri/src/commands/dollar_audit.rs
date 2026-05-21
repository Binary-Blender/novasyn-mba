// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DollarAudit {
    pub id: String,
    pub business_id: i64,
    pub activity_name: String,
    pub tier: String,
    pub hours_per_week: f64,
    pub ai_transferable: i64,
    pub transfer_status: String,
    pub notes: Option<String>,
    pub business_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDollarAuditInput {
    pub business_id: i64,
    pub activity_name: String,
    pub tier: Option<String>,
    pub hours_per_week: Option<f64>,
    pub ai_transferable: Option<i64>,
    pub transfer_status: Option<String>,
    pub notes: Option<String>,
    pub business_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDollarAuditInput {
    pub business_id: Option<i64>,
    pub activity_name: Option<String>,
    pub tier: Option<String>,
    pub hours_per_week: Option<f64>,
    pub ai_transferable: Option<i64>,
    pub transfer_status: Option<String>,
    pub notes: Option<String>,
}

impl DollarAudit {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            activity_name: row.get("activity_name").unwrap(),
            tier: row.get("tier").unwrap(),
            hours_per_week: row.get("hours_per_week").unwrap(),
            ai_transferable: row.get("ai_transferable").unwrap(),
            transfer_status: row.get("transfer_status").unwrap(),
            notes: row.get("notes").ok(),
            business_id: row.get("business_id").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_dollar_audits(db: tauri::State<'_, DbPool>) -> Result<Vec<DollarAudit>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM dollar_audits ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(DollarAudit::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_dollar_audit(db: tauri::State<'_, DbPool>, input: CreateDollarAuditInput) -> Result<DollarAudit, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO dollar_audits (id, business_id, activity_name, tier, hours_per_week, ai_transferable, transfer_status, notes, business_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.activity_name,
            input.tier.unwrap_or("twenty".to_string()),
            input.hours_per_week.unwrap_or(0),
            input.ai_transferable.unwrap_or(0),
            input.transfer_status.unwrap_or("not_started".to_string()),
            input.notes,
            input.business_id,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_dollar_audit(db, id)
}

#[tauri::command]
pub fn get_dollar_audit(db: tauri::State<'_, DbPool>, id: String) -> Result<DollarAudit, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM dollar_audits WHERE id = ?", [&id], |row| {
        Ok(DollarAudit::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_dollar_audit(db: tauri::State<'_, DbPool>, id: String, input: UpdateDollarAuditInput) -> Result<DollarAudit, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.activity_name {
        sets.push("activity_name = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.tier {
        sets.push("tier = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.hours_per_week {
        sets.push("hours_per_week = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.ai_transferable {
        sets.push("ai_transferable = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.transfer_status {
        sets.push("transfer_status = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.notes {
        sets.push("notes = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE dollar_audits SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_dollar_audit(db, id)
}

#[tauri::command]
pub fn delete_dollar_audit(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM dollar_audits WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
