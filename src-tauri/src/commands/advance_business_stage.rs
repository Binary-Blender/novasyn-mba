// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceBusinessStageInput {
    pub business_id: String,
    pub new_stage: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceBusinessStageOutput {
    pub success: i64,
    pub message: String,
}

#[tauri::command]
pub async fn advance_business_stage(
    input: AdvanceBusinessStageInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<AdvanceBusinessStageOutput, String> {
    let conn = _db.lock().map_err(|e| e.to_string())?;

    // 1. Fetch current stage
    let current_stage: String = conn
        .query_row(
            "SELECT stage FROM businesses WHERE id = ?1",
            rusqlite::params![input.business_id],
            |row| row.get(0),
        )
        .map_err(|_| "Business not found".to_string())?;

    let new_stage = input.new_stage.as_str();
    let bid = &input.business_id;

    // 2. Validate transition
    let exit_stages = ["Paused", "Killed", "Exited"];
    let valid_forward = match current_stage.as_str() {
        "Idea"      => vec!["Validated"],
        "Validated" => vec!["Building"],
        "Building"  => vec!["Active"],
        "Active"    => vec!["Scaling"],
        _           => vec![],
    };

    let is_exit = exit_stages.contains(&new_stage);
    let is_valid_forward = valid_forward.contains(&new_stage);

    if !is_exit && !is_valid_forward {
        return Ok(AdvanceBusinessStageOutput {
            success: 0,
            message: format!(
                "Invalid transition: {} → {}. Valid next stages: {}{}",
                current_stage,
                new_stage,
                valid_forward.join(", "),
                if valid_forward.is_empty() { "" } else { ", Paused, Killed, Exited" },
            ),
        });
    }

    // 3. Gate checks (exit stages have no gates)
    if !is_exit {
        match (current_stage.as_str(), new_stage) {
            ("Idea", "Validated") => {
                // strategic_positions must have value_proposition AND target_market
                let sp_ok: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM strategic_positions \
                     WHERE business_id = ?1 \
                       AND value_proposition IS NOT NULL \
                       AND target_market IS NOT NULL",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                if sp_ok == 0 {
                    return Ok(AdvanceBusinessStageOutput {
                        success: 0,
                        message: "Gate not met: add a Strategic Position with both value_proposition and target_market filled in.".to_string(),
                    });
                }

                let contact_count: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM sales_contacts WHERE business_id = ?1",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                if contact_count < 1 {
                    return Ok(AdvanceBusinessStageOutput {
                        success: 0,
                        message: "Gate not met: add at least 1 sales contact before validating.".to_string(),
                    });
                }
            }

            ("Validated", "Building") => {
                let notes_count: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM sales_contacts \
                     WHERE business_id = ?1 AND notes IS NOT NULL",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                if notes_count < 1 {
                    return Ok(AdvanceBusinessStageOutput {
                        success: 0,
                        message: "Gate not met: at least one sales contact must have notes recorded (evidence of real conversations).".to_string(),
                    });
                }
            }

            ("Building", "Active") => {
                let revenue_ok: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM financial_snapshots \
                     WHERE business_id = ?1 AND revenue > 0",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                let closed_won: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM sales_contacts \
                     WHERE business_id = ?1 AND stage = 'closed_won'",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                if revenue_ok == 0 && closed_won < 3 {
                    return Ok(AdvanceBusinessStageOutput {
                        success: 0,
                        message: "Gate not met: record a financial snapshot with revenue > 0, OR close at least 3 sales contacts (stage = closed_won).".to_string(),
                    });
                }
            }

            ("Active", "Scaling") => {
                let snapshot_count: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM financial_snapshots WHERE business_id = ?1",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                let ops_ok: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM operations_audits \
                     WHERE business_id = ?1 AND overall_score < 20",
                    rusqlite::params![bid],
                    |row| row.get(0),
                ).unwrap_or(0);

                if snapshot_count < 2 {
                    return Ok(AdvanceBusinessStageOutput {
                        success: 0,
                        message: "Gate not met: need at least 2 financial snapshots to demonstrate consistent revenue.".to_string(),
                    });
                }

                if ops_ok == 0 {
                    return Ok(AdvanceBusinessStageOutput {
                        success: 0,
                        message: "Gate not met: need an operations audit with overall_score < 20 (demonstrating bottleneck awareness).".to_string(),
                    });
                }
            }

            _ => {}
        }
    }

    // 4. Apply the stage update
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE businesses SET stage = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![new_stage, now, bid],
    ).map_err(|e| e.to_string())?;

    Ok(AdvanceBusinessStageOutput {
        success: 1,
        message: format!("Stage advanced to {}", new_stage),
    })
}
