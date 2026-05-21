// EVENT BUS — static event registry with typed emit
// @agicore-generated — DO NOT EDIT (regenerate via: agicore compile)

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

// ─── Event registry ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventInfo {
    pub name: &'static str,
    pub idempotent: bool,
    pub ttl: u64,
}

const EVENT_REGISTRY: &[EventInfo] = &[
    EventInfo {
        name: "weekly_review_reminder",
        idempotent: false,
        ttl: 0,
    },
    EventInfo {
        name: "monthly_financial_snapshot",
        idempotent: false,
        ttl: 0,
    },
    EventInfo {
        name: "quarterly_strategy_review",
        idempotent: false,
        ttl: 0,
    },
];

// ─── Emit helper ─────────────────────────────────────────────────────────────

pub fn emit_event(app: &AppHandle, event_name: &str, payload: serde_json::Value) {
    if let Err(e) = app.emit(event_name, payload) {
        eprintln!("[EventBus] Failed to emit {}: {}", event_name, e);
    }
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_event_registry() -> Vec<EventInfo> {
    EVENT_REGISTRY.iter().cloned().collect()
}
