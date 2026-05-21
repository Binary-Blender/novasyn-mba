// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCsvContactsInput {
    pub business_id: String,
    pub csv_path: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCsvContactsOutput {
    pub imported_count: i64,
    pub skipped_count: i64,
    pub error_summary: String,
}

const VALID_STAGES: &[&str] = &[
    "prospect", "qualified", "proposal", "closed_won", "closed_lost", "referred",
];

#[tauri::command]
pub async fn import_csv_contacts(
    input: ImportCsvContactsInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<ImportCsvContactsOutput, String> {
    // 1. Read the file
    let file_content = std::fs::read_to_string(&input.csv_path)
        .map_err(|e| format!("Failed to read CSV file: {}", e))?;

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(file_content.as_bytes());

    // Build a case-insensitive header map
    let headers = rdr
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?
        .clone();

    let header_map: std::collections::HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.to_lowercase(), i))
        .collect();

    let get_field = |record: &csv::StringRecord, key: &str| -> Option<String> {
        header_map
            .get(key)
            .and_then(|&i| record.get(i))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    let conn = _db.lock().map_err(|e| e.to_string())?;

    let mut imported_count: i64 = 0;
    let mut skipped_count: i64 = 0;
    let mut errors: Vec<String> = Vec::new();

    for (row_num, result) in rdr.records().enumerate() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                if errors.len() < 10 {
                    errors.push(format!("Row {}: parse error — {}", row_num + 2, e));
                }
                skipped_count += 1;
                continue;
            }
        };

        // name is required
        let name = match get_field(&record, "name") {
            Some(n) => n,
            None => {
                if errors.len() < 10 {
                    errors.push(format!("Row {}: skipped — missing required field 'name'", row_num + 2));
                }
                skipped_count += 1;
                continue;
            }
        };

        let company = get_field(&record, "company");
        let source = get_field(&record, "source");
        let next_action = get_field(&record, "next_action");
        let next_action_date = get_field(&record, "next_action_date");
        let notes = get_field(&record, "notes");

        // ltv_potential: parse as f64, default 0
        let ltv_potential: f64 = get_field(&record, "ltv_potential")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        // stage: validate or default to "prospect"
        let stage = match get_field(&record, "stage") {
            Some(s) => {
                let lower = s.to_lowercase();
                if VALID_STAGES.contains(&lower.as_str()) {
                    lower
                } else {
                    if errors.len() < 10 {
                        errors.push(format!(
                            "Row {}: invalid stage '{}' — defaulted to 'prospect'",
                            row_num + 2,
                            s
                        ));
                    }
                    "prospect".to_string()
                }
            }
            None => "prospect".to_string(),
        };

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match conn.execute(
            "INSERT INTO sales_contacts \
             (id, business_id, name, company, stage, ltv_potential, source, next_action, next_action_date, notes, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                id,
                input.business_id,
                name,
                company,
                stage,
                ltv_potential,
                source,
                next_action,
                next_action_date,
                notes,
                now,
                now,
            ],
        ) {
            Ok(_) => imported_count += 1,
            Err(e) => {
                if errors.len() < 10 {
                    errors.push(format!("Row {}: DB insert failed — {}", row_num + 2, e));
                }
                skipped_count += 1;
            }
        }
    }

    let error_summary = if errors.is_empty() {
        String::new()
    } else {
        errors.join("\n")
    };

    Ok(ImportCsvContactsOutput {
        imported_count,
        skipped_count,
        error_summary,
    })
}
