// Agicore Generated — DO NOT EDIT BY HAND
// Re-run `agicore generate` to regenerate.
// Compiler commands: document file I/O + semantic "Send To" transitions.

#![allow(unused_variables, dead_code)]

use serde::{Deserialize, Serialize};
use crate::db::DbPool;
use crate::ai_service::ApiKeyStore;

// ── Document file I/O ────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedDocument {
    pub path: String,
    pub title: String,
    pub size: u64,
    pub modified_at: String,
}

#[tauri::command]
pub fn read_document_content(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_document_content(file_path: String, content: String) -> Result<(), String> {
    if let Some(parent) = std::path::PathBuf::from(&file_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&file_path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn scan_documents_dir(dir: String) -> Result<Vec<ScannedDocument>, String> {
    let path = std::path::PathBuf::from(&dir);
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut docs = Vec::new();
    scan_recursive(&path, &mut docs)?;
    Ok(docs)
}

fn scan_recursive(dir: &std::path::PathBuf, out: &mut Vec<ScannedDocument>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_recursive(&path, out)?;
        } else if let Some(ext) = path.extension() {
            if ext == "md" || ext == "txt" || ext == "markdown" {
                let meta = entry.metadata().map_err(|e| e.to_string())?;
                let title = path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let modified_at = meta.modified().ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + d)
                        .to_rfc3339())
                    .unwrap_or_default();
                out.push(ScannedDocument {
                    path: path.to_string_lossy().to_string(),
                    title,
                    size: meta.len(),
                    modified_at,
                });
            }
        }
    }
    Ok(())
}

// ── AI call (non-streaming) ───────────────────────────────────────────────────

async fn compiler_call_ai(
    model: &str,
    system: &str,
    user_msg: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let empty = String::new();
    let client = reqwest::Client::new();
    let messages = vec![serde_json::json!({"role": "user", "content": user_msg})];

    if model.starts_with("claude-") {
        let key = keys.get("anthropic").unwrap_or(&empty);
        let body = serde_json::json!({
            "model": model, "max_tokens": 8192, "stream": false,
            "system": system, "messages": messages,
        });
        let res = client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        return json["content"][0]["text"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("anthropic: unexpected response: {:?}", json));
    }

    if model.starts_with("gemini-") {
        let key = keys.get("google").unwrap_or(&empty);
        let contents = vec![serde_json::json!({"role": "user", "parts": [{"text": user_msg}]})];
        let body = serde_json::json!({
            "systemInstruction": {"parts": [{"text": system}]},
            "contents": contents,
            "generationConfig": {"maxOutputTokens": 8192}
        });
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, key
        );
        let res = client.post(&url).header("Content-Type", "application/json")
            .json(&body).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        return json["candidates"][0]["content"]["parts"][0]["text"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("google: unexpected response: {:?}", json));
    }

    // OpenAI-compatible (openai, xai, huggingface)
    let (url, key) = if model.starts_with("grok-") {
        ("https://api.x.ai/v1/chat/completions", keys.get("xai").unwrap_or(&empty))
    } else {
        ("https://api.openai.com/v1/chat/completions", keys.get("openai").unwrap_or(&empty))
    };
    let mut msgs = vec![serde_json::json!({"role": "system", "content": system})];
    msgs.extend(messages);
    let body = serde_json::json!({"model": model, "stream": false, "messages": msgs});
    let res = client.post(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .json(&body).send().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    json["choices"][0]["message"]["content"].as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("openai: unexpected response: {:?}", json))
}

// ── reviews_to_action_plan ──
// Distill recent weekly reviews into a prioritized action plan

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileReviewsToActionPlanInput {
    pub message_ids: serde_json::Value,
}

#[tauri::command]
pub async fn reviews_to_action_plan(
    input: CompileReviewsToActionPlanInput,
    db: tauri::State<'_, DbPool>,
) -> Result<serde_json::Value, String> {
    let _ = (input, db);
    Err("reviews_to_action_plan: not yet implemented".to_string())
}

// ── advisor_outputs_to_priorities ──
// Compile multiple discipline advisor briefings into a ranked priority list

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileAdvisorOutputsToPrioritiesInput {
    pub message_ids: serde_json::Value,
}

#[tauri::command]
pub async fn advisor_outputs_to_priorities(
    input: CompileAdvisorOutputsToPrioritiesInput,
    db: tauri::State<'_, DbPool>,
) -> Result<serde_json::Value, String> {
    let _ = (input, db);
    Err("advisor_outputs_to_priorities: not yet implemented".to_string())
}

// ── financial_data_to_report ──
// Transform FinancialSnapshot records into a structured business health report

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileFinancialDataToReportInput {
    pub message_ids: serde_json::Value,
}

#[tauri::command]
pub async fn financial_data_to_report(
    input: CompileFinancialDataToReportInput,
    db: tauri::State<'_, DbPool>,
) -> Result<serde_json::Value, String> {
    let _ = (input, db);
    Err("financial_data_to_report: not yet implemented".to_string())
}

// ── business_data_to_pitch ──
// Transform structured business entity data into a pitch narrative outline

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileBusinessDataToPitchInput {
    pub message_ids: serde_json::Value,
}

#[tauri::command]
pub async fn business_data_to_pitch(
    input: CompileBusinessDataToPitchInput,
    db: tauri::State<'_, DbPool>,
) -> Result<serde_json::Value, String> {
    let _ = (input, db);
    Err("business_data_to_pitch: not yet implemented".to_string())
}
