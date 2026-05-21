// @agicore-protected — fill in your Rust logic; this file won't be overwritten
use serde::{Deserialize, Serialize};
use crate::db::DbPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCsvContactsInput {
    pub business_id: i64,
    pub csv_path: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCsvContactsOutput {
    pub imported_count: i64,
    pub skipped_count: i64,
    pub error_summary: String,
}

#[tauri::command]
pub async fn import_csv_contacts(
    input: ImportCsvContactsInput,
    _db: tauri::State<'_, DbPool>,
) -> Result<ImportCsvContactsOutput, String> {
    // TODO: implement import_csv_contacts
    todo!()
}
