// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardMember {
    pub id: String,
    pub name: String,
    pub expertise_domain: Option<String>,
    pub relationship_type: String,
    pub last_consulted: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBoardMemberInput {
    pub name: String,
    pub expertise_domain: Option<String>,
    pub relationship_type: Option<String>,
    pub last_consulted: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBoardMemberInput {
    pub name: Option<String>,
    pub expertise_domain: Option<String>,
    pub relationship_type: Option<String>,
    pub last_consulted: Option<String>,
    pub notes: Option<String>,
}

impl BoardMember {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            name: row.get("name").unwrap(),
            expertise_domain: row.get("expertise_domain").ok(),
            relationship_type: row.get("relationship_type").unwrap(),
            last_consulted: row.get("last_consulted").ok(),
            notes: row.get("notes").ok(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_board_members(db: tauri::State<'_, DbPool>) -> Result<Vec<BoardMember>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM board_members ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(BoardMember::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_board_member(db: tauri::State<'_, DbPool>, input: CreateBoardMemberInput) -> Result<BoardMember, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO board_members (id, name, expertise_domain, relationship_type, last_consulted, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.name,
            input.expertise_domain,
            input.relationship_type.unwrap_or("mentor".to_string()),
            input.last_consulted,
            input.notes,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_board_member(db, id)
}

#[tauri::command]
pub fn get_board_member(db: tauri::State<'_, DbPool>, id: String) -> Result<BoardMember, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM board_members WHERE id = ?", [&id], |row| {
        Ok(BoardMember::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_board_member(db: tauri::State<'_, DbPool>, id: String, input: UpdateBoardMemberInput) -> Result<BoardMember, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.name {
        sets.push("name = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.expertise_domain {
        sets.push("expertise_domain = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.relationship_type {
        sets.push("relationship_type = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.last_consulted {
        sets.push("last_consulted = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.notes {
        sets.push("notes = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE board_members SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_board_member(db, id)
}

#[tauri::command]
pub fn delete_board_member(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM board_members WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
