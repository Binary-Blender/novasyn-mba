// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIStackDecision {
    pub id: String,
    pub business_id: String,
    pub capability: String,
    pub decision: String,
    pub tool_or_approach: Option<String>,
    pub cost_per_month: f64,
    pub is_core_capability: i64,
    pub review_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAIStackDecisionInput {
    pub business_id: String,
    pub capability: String,
    pub decision: Option<String>,
    pub tool_or_approach: Option<String>,
    pub cost_per_month: Option<f64>,
    pub is_core_capability: Option<i64>,
    pub review_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAIStackDecisionInput {
    pub business_id: Option<String>,
    pub capability: Option<String>,
    pub decision: Option<String>,
    pub tool_or_approach: Option<String>,
    pub cost_per_month: Option<f64>,
    pub is_core_capability: Option<i64>,
    pub review_date: Option<String>,
}

impl AIStackDecision {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            capability: row.get("capability").unwrap(),
            decision: row.get("decision").unwrap(),
            tool_or_approach: row.get("tool_or_approach").ok(),
            cost_per_month: row.get("cost_per_month").unwrap(),
            is_core_capability: row.get("is_core_capability").unwrap(),
            review_date: row.get("review_date").ok(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_a_i_stack_decisions(db: tauri::State<'_, DbPool>) -> Result<Vec<AIStackDecision>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM a_i_stack_decisions ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(AIStackDecision::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_a_i_stack_decision(db: tauri::State<'_, DbPool>, input: CreateAIStackDecisionInput) -> Result<AIStackDecision, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO a_i_stack_decisions (id, business_id, capability, decision, tool_or_approach, cost_per_month, is_core_capability, review_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.capability,
            input.decision.unwrap_or("buy".to_string()),
            input.tool_or_approach,
            input.cost_per_month.unwrap_or(0.0),
            input.is_core_capability.unwrap_or(0),
            input.review_date,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_a_i_stack_decision(db, id)
}

#[tauri::command]
pub fn get_a_i_stack_decision(db: tauri::State<'_, DbPool>, id: String) -> Result<AIStackDecision, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM a_i_stack_decisions WHERE id = ?", [&id], |row| {
        Ok(AIStackDecision::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_a_i_stack_decision(db: tauri::State<'_, DbPool>, id: String, input: UpdateAIStackDecisionInput) -> Result<AIStackDecision, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.capability {
        sets.push("capability = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.decision {
        sets.push("decision = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.tool_or_approach {
        sets.push("tool_or_approach = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.cost_per_month {
        sets.push("cost_per_month = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.is_core_capability {
        sets.push("is_core_capability = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.review_date {
        sets.push("review_date = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE a_i_stack_decisions SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_a_i_stack_decision(db, id)
}

#[tauri::command]
pub fn delete_a_i_stack_decision(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM a_i_stack_decisions WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
