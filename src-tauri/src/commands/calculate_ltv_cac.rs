// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateLtvCacInput {
    pub snapshot_id: i64,
    pub ltv: f64,
    pub cac: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateLtvCacOutput {
    pub ltv_cac_ratio: f64,
    pub health_flag: String,
}

#[tauri::command]
pub async fn calculate_ltv_cac(
    input: CalculateLtvCacInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<CalculateLtvCacOutput, String> {
    let ratio = if input.cac > 0.0 {
        input.ltv / input.cac
    } else {
        0.0
    };

    let health_flag = if ratio >= 3.0 {
        "healthy".to_string()
    } else if ratio >= 2.0 {
        "warning".to_string()
    } else {
        "critical".to_string()
    };

    Ok(CalculateLtvCacOutput {
        ltv_cac_ratio: ratio,
        health_flag,
    })
}
