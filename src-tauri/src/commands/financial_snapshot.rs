// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialSnapshot {
    pub id: String,
    pub business_id: String,
    pub period: String,
    pub revenue: f64,
    pub cac: f64,
    pub ltv: f64,
    pub ltv_cac_ratio: f64,
    pub gross_margin_pct: f64,
    pub mrr: f64,
    pub cash_runway_months: f64,
    pub recorded_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFinancialSnapshotInput {
    pub business_id: String,
    pub period: String,
    pub revenue: Option<f64>,
    pub cac: Option<f64>,
    pub ltv: Option<f64>,
    pub ltv_cac_ratio: Option<f64>,
    pub gross_margin_pct: Option<f64>,
    pub mrr: Option<f64>,
    pub cash_runway_months: Option<f64>,
    pub recorded_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFinancialSnapshotInput {
    pub business_id: Option<String>,
    pub period: Option<String>,
    pub revenue: Option<f64>,
    pub cac: Option<f64>,
    pub ltv: Option<f64>,
    pub ltv_cac_ratio: Option<f64>,
    pub gross_margin_pct: Option<f64>,
    pub mrr: Option<f64>,
    pub cash_runway_months: Option<f64>,
    pub recorded_at: Option<String>,
}

impl FinancialSnapshot {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            period: row.get("period").unwrap(),
            revenue: row.get("revenue").unwrap(),
            cac: row.get("cac").unwrap(),
            ltv: row.get("ltv").unwrap(),
            ltv_cac_ratio: row.get("ltv_cac_ratio").unwrap(),
            gross_margin_pct: row.get("gross_margin_pct").unwrap(),
            mrr: row.get("mrr").unwrap(),
            cash_runway_months: row.get("cash_runway_months").unwrap(),
            recorded_at: row.get("recorded_at").ok(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_financial_snapshots(db: tauri::State<'_, DbPool>) -> Result<Vec<FinancialSnapshot>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM financial_snapshots ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(FinancialSnapshot::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_financial_snapshot(db: tauri::State<'_, DbPool>, input: CreateFinancialSnapshotInput) -> Result<FinancialSnapshot, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO financial_snapshots (id, business_id, period, revenue, cac, ltv, ltv_cac_ratio, gross_margin_pct, mrr, cash_runway_months, recorded_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.period,
            input.revenue.unwrap_or(0.0),
            input.cac.unwrap_or(0.0),
            input.ltv.unwrap_or(0.0),
            input.ltv_cac_ratio.unwrap_or(0.0),
            input.gross_margin_pct.unwrap_or(0.0),
            input.mrr.unwrap_or(0.0),
            input.cash_runway_months.unwrap_or(0.0),
            input.recorded_at,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_financial_snapshot(db, id)
}

#[tauri::command]
pub fn get_financial_snapshot(db: tauri::State<'_, DbPool>, id: String) -> Result<FinancialSnapshot, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM financial_snapshots WHERE id = ?", [&id], |row| {
        Ok(FinancialSnapshot::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_financial_snapshot(db: tauri::State<'_, DbPool>, id: String, input: UpdateFinancialSnapshotInput) -> Result<FinancialSnapshot, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.period {
        sets.push("period = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.revenue {
        sets.push("revenue = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.cac {
        sets.push("cac = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.ltv {
        sets.push("ltv = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.ltv_cac_ratio {
        sets.push("ltv_cac_ratio = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.gross_margin_pct {
        sets.push("gross_margin_pct = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.mrr {
        sets.push("mrr = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.cash_runway_months {
        sets.push("cash_runway_months = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.recorded_at {
        sets.push("recorded_at = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE financial_snapshots SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_financial_snapshot(db, id)
}

#[tauri::command]
pub fn delete_financial_snapshot(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM financial_snapshots WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
