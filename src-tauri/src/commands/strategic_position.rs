// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategicPosition {
    pub id: String,
    pub business_id: String,
    pub target_market: Option<String>,
    pub value_proposition: Option<String>,
    pub primary_moat: String,
    pub competitors: Option<String>,
    pub unfair_advantages: Option<String>,
    pub moat_score: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStrategicPositionInput {
    pub business_id: String,
    pub target_market: Option<String>,
    pub value_proposition: Option<String>,
    pub primary_moat: Option<String>,
    pub competitors: Option<String>,
    pub unfair_advantages: Option<String>,
    pub moat_score: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStrategicPositionInput {
    pub business_id: Option<String>,
    pub target_market: Option<String>,
    pub value_proposition: Option<String>,
    pub primary_moat: Option<String>,
    pub competitors: Option<String>,
    pub unfair_advantages: Option<String>,
    pub moat_score: Option<i64>,
}

impl StrategicPosition {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").unwrap(),
            target_market: row.get("target_market").ok(),
            value_proposition: row.get("value_proposition").ok(),
            primary_moat: row.get("primary_moat").unwrap(),
            competitors: row.get("competitors").ok(),
            unfair_advantages: row.get("unfair_advantages").ok(),
            moat_score: row.get("moat_score").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_strategic_positions(db: tauri::State<'_, DbPool>) -> Result<Vec<StrategicPosition>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM strategic_positions ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(StrategicPosition::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_strategic_position(db: tauri::State<'_, DbPool>, input: CreateStrategicPositionInput) -> Result<StrategicPosition, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO strategic_positions (id, business_id, target_market, value_proposition, primary_moat, competitors, unfair_advantages, moat_score, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.target_market,
            input.value_proposition,
            input.primary_moat.unwrap_or("reputation".to_string()),
            input.competitors,
            input.unfair_advantages,
            input.moat_score.unwrap_or(5),
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_strategic_position(db, id)
}

#[tauri::command]
pub fn get_strategic_position(db: tauri::State<'_, DbPool>, id: String) -> Result<StrategicPosition, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM strategic_positions WHERE id = ?", [&id], |row| {
        Ok(StrategicPosition::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_strategic_position(db: tauri::State<'_, DbPool>, id: String, input: UpdateStrategicPositionInput) -> Result<StrategicPosition, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.target_market {
        sets.push("target_market = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.value_proposition {
        sets.push("value_proposition = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.primary_moat {
        sets.push("primary_moat = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.competitors {
        sets.push("competitors = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.unfair_advantages {
        sets.push("unfair_advantages = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.moat_score {
        sets.push("moat_score = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE strategic_positions SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_strategic_position(db, id)
}

#[tauri::command]
pub fn delete_strategic_position(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM strategic_positions WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
