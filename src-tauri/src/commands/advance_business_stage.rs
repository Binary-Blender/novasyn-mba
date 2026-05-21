// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceBusinessStageInput {
    pub business_id: i64,
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
    // TODO: implement advance_business_stage
    todo!()
}
