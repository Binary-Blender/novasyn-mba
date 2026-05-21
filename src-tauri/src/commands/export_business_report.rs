// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportBusinessReportInput {
    pub business_id: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportBusinessReportOutput {
    pub file_path: String,
    pub success: i64,
}

#[tauri::command]
pub async fn export_business_report(
    input: ExportBusinessReportInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<ExportBusinessReportOutput, String> {
    // TODO: implement export_business_report
    todo!()
}
