// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesContact {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub company: Option<String>,
    pub stage: String,
    pub ltv_potential: f64,
    pub source: String,
    pub next_action: Option<String>,
    pub next_action_date: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSalesContactInput {
    pub business_id: String,
    pub name: String,
    pub company: Option<String>,
    pub stage: Option<String>,
    pub ltv_potential: Option<f64>,
    pub source: Option<String>,
    pub next_action: Option<String>,
    pub next_action_date: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSalesContactInput {
    pub business_id: Option<String>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub stage: Option<String>,
    pub ltv_potential: Option<f64>,
    pub source: Option<String>,
    pub next_action: Option<String>,
    pub next_action_date: Option<String>,
    pub notes: Option<String>,
}

impl SalesContact {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            name: row.get("name").unwrap(),
            company: row.get("company").ok(),
            stage: row.get("stage").unwrap(),
            ltv_potential: row.get("ltv_potential").unwrap(),
            source: row.get("source").unwrap(),
            next_action: row.get("next_action").ok(),
            next_action_date: row.get("next_action_date").ok(),
            notes: row.get("notes").ok(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_sales_contacts(db: tauri::State<'_, DbPool>) -> Result<Vec<SalesContact>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM sales_contacts ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(SalesContact::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_sales_contact(db: tauri::State<'_, DbPool>, input: CreateSalesContactInput) -> Result<SalesContact, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO sales_contacts (id, business_id, name, company, stage, ltv_potential, source, next_action, next_action_date, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.name,
            input.company,
            input.stage.unwrap_or("prospect".to_string()),
            input.ltv_potential.unwrap_or(0.0),
            input.source.unwrap_or("inbound".to_string()),
            input.next_action,
            input.next_action_date,
            input.notes,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_sales_contact(db, id)
}

#[tauri::command]
pub fn get_sales_contact(db: tauri::State<'_, DbPool>, id: String) -> Result<SalesContact, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM sales_contacts WHERE id = ?", [&id], |row| {
        Ok(SalesContact::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_sales_contact(db: tauri::State<'_, DbPool>, id: String, input: UpdateSalesContactInput) -> Result<SalesContact, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.name {
        sets.push("name = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.company {
        sets.push("company = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.stage {
        sets.push("stage = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.ltv_potential {
        sets.push("ltv_potential = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.source {
        sets.push("source = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.next_action {
        sets.push("next_action = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.next_action_date {
        sets.push("next_action_date = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.notes {
        sets.push("notes = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE sales_contacts SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_sales_contact(db, id)
}

#[tauri::command]
pub fn delete_sales_contact(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM sales_contacts WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
