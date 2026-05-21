// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationsAudit {
    pub id: String,
    pub business_id: String,
    pub knowledge_bottleneck: i64,
    pub quality_bottleneck: i64,
    pub coordination_bottleneck: i64,
    pub communication_bottleneck: i64,
    pub creative_bottleneck: i64,
    pub decision_bottleneck: i64,
    pub overall_score: i64,
    pub audit_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOperationsAuditInput {
    pub business_id: String,
    pub knowledge_bottleneck: Option<i64>,
    pub quality_bottleneck: Option<i64>,
    pub coordination_bottleneck: Option<i64>,
    pub communication_bottleneck: Option<i64>,
    pub creative_bottleneck: Option<i64>,
    pub decision_bottleneck: Option<i64>,
    pub overall_score: Option<i64>,
    pub audit_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOperationsAuditInput {
    pub business_id: Option<String>,
    pub knowledge_bottleneck: Option<i64>,
    pub quality_bottleneck: Option<i64>,
    pub coordination_bottleneck: Option<i64>,
    pub communication_bottleneck: Option<i64>,
    pub creative_bottleneck: Option<i64>,
    pub decision_bottleneck: Option<i64>,
    pub overall_score: Option<i64>,
    pub audit_date: Option<String>,
}

impl OperationsAudit {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            knowledge_bottleneck: row.get("knowledge_bottleneck").unwrap(),
            quality_bottleneck: row.get("quality_bottleneck").unwrap(),
            coordination_bottleneck: row.get("coordination_bottleneck").unwrap(),
            communication_bottleneck: row.get("communication_bottleneck").unwrap(),
            creative_bottleneck: row.get("creative_bottleneck").unwrap(),
            decision_bottleneck: row.get("decision_bottleneck").unwrap(),
            overall_score: row.get("overall_score").unwrap(),
            audit_date: row.get("audit_date").ok(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_operations_audits(db: tauri::State<'_, DbPool>) -> Result<Vec<OperationsAudit>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM operations_audits ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(OperationsAudit::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_operations_audit(db: tauri::State<'_, DbPool>, input: CreateOperationsAuditInput) -> Result<OperationsAudit, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO operations_audits (id, business_id, knowledge_bottleneck, quality_bottleneck, coordination_bottleneck, communication_bottleneck, creative_bottleneck, decision_bottleneck, overall_score, audit_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.knowledge_bottleneck.unwrap_or(3),
            input.quality_bottleneck.unwrap_or(3),
            input.coordination_bottleneck.unwrap_or(3),
            input.communication_bottleneck.unwrap_or(3),
            input.creative_bottleneck.unwrap_or(3),
            input.decision_bottleneck.unwrap_or(3),
            input.overall_score.unwrap_or(18),
            input.audit_date,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_operations_audit(db, id)
}

#[tauri::command]
pub fn get_operations_audit(db: tauri::State<'_, DbPool>, id: String) -> Result<OperationsAudit, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM operations_audits WHERE id = ?", [&id], |row| {
        Ok(OperationsAudit::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_operations_audit(db: tauri::State<'_, DbPool>, id: String, input: UpdateOperationsAuditInput) -> Result<OperationsAudit, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.knowledge_bottleneck {
        sets.push("knowledge_bottleneck = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.quality_bottleneck {
        sets.push("quality_bottleneck = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.coordination_bottleneck {
        sets.push("coordination_bottleneck = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.communication_bottleneck {
        sets.push("communication_bottleneck = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.creative_bottleneck {
        sets.push("creative_bottleneck = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.decision_bottleneck {
        sets.push("decision_bottleneck = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.overall_score {
        sets.push("overall_score = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.audit_date {
        sets.push("audit_date = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE operations_audits SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_operations_audit(db, id)
}

#[tauri::command]
pub fn delete_operations_audit(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM operations_audits WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
