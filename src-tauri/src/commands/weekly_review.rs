// Agicore Generated Rust Code
// App: novasyn_mba

use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyReview {
    pub id: String,
    pub business_id: Option<i64>,
    pub week_of: String,
    pub accomplishments: Option<String>,
    pub learnings: Option<String>,
    pub next_week_focus: Option<String>,
    pub energy_level: i64,
    pub ceo_time_hours: f64,
    pub business_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWeeklyReviewInput {
    pub business_id: Option<i64>,
    pub week_of: String,
    pub accomplishments: Option<String>,
    pub learnings: Option<String>,
    pub next_week_focus: Option<String>,
    pub energy_level: Option<i64>,
    pub ceo_time_hours: Option<f64>,
    pub business_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWeeklyReviewInput {
    pub business_id: Option<i64>,
    pub week_of: Option<String>,
    pub accomplishments: Option<String>,
    pub learnings: Option<String>,
    pub next_week_focus: Option<String>,
    pub energy_level: Option<i64>,
    pub ceo_time_hours: Option<f64>,
}

impl WeeklyReview {
    fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            business_id: row.get("business_id").ok(),
            week_of: row.get("week_of").unwrap(),
            accomplishments: row.get("accomplishments").ok(),
            learnings: row.get("learnings").ok(),
            next_week_focus: row.get("next_week_focus").ok(),
            energy_level: row.get("energy_level").unwrap(),
            ceo_time_hours: row.get("ceo_time_hours").unwrap(),
            business_id: row.get("business_id").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        }
    }
}

#[tauri::command]
pub fn list_weekly_reviews(db: tauri::State<'_, DbPool>) -> Result<Vec<WeeklyReview>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM weekly_reviews ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(WeeklyReview::from_row(row)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn create_weekly_review(db: tauri::State<'_, DbPool>, input: CreateWeeklyReviewInput) -> Result<WeeklyReview, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO weekly_reviews (id, business_id, week_of, accomplishments, learnings, next_week_focus, energy_level, ceo_time_hours, business_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            input.business_id,
            input.week_of,
            input.accomplishments,
            input.learnings,
            input.next_week_focus,
            input.energy_level.unwrap_or(3),
            input.ceo_time_hours.unwrap_or(0),
            input.business_id,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    get_weekly_review(db, id)
}

#[tauri::command]
pub fn get_weekly_review(db: tauri::State<'_, DbPool>, id: String) -> Result<WeeklyReview, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT * FROM weekly_reviews WHERE id = ?", [&id], |row| {
        Ok(WeeklyReview::from_row(row))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_weekly_review(db: tauri::State<'_, DbPool>, id: String, input: UpdateWeeklyReviewInput) -> Result<WeeklyReview, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(val) = input.business_id {
        sets.push("business_id = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.week_of {
        sets.push("week_of = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.accomplishments {
        sets.push("accomplishments = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.learnings {
        sets.push("learnings = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.next_week_focus {
        sets.push("next_week_focus = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.energy_level {
        sets.push("energy_level = ?".to_string());
        params.push(Box::new(val));
    }
    if let Some(val) = input.ceo_time_hours {
        sets.push("ceo_time_hours = ?".to_string());
        params.push(Box::new(val));
    }
    sets.push("updated_at = ?".to_string());
    params.push(Box::new(chrono::Utc::now().to_rfc3339()));
    params.push(Box::new(id.clone()));
    let sql = format!("UPDATE weekly_reviews SET {} WHERE id = ?", sets.join(", "));
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;
    drop(conn);
    get_weekly_review(db, id)
}

#[tauri::command]
pub fn delete_weekly_review(db: tauri::State<'_, DbPool>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM weekly_reviews WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
