// EVENT BUS — typed listen/emit helpers
// @agicore-generated — DO NOT EDIT (regenerate via: agicore compile)

import { listen, emit } from '@tauri-apps/api/event';

// ─── weekly_review_reminder ─────────────────────────────────────────────────────────────
// Monday morning reminder to complete the weekly business review

export interface weekly_review_reminderPayload {

}

export function listenweekly_review_reminder(cb: (payload: weekly_review_reminderPayload) => void) {
  return listen<weekly_review_reminderPayload>('weekly_review_reminder', (event) => cb(event.payload));
}

export function emitweekly_review_reminder(payload: weekly_review_reminderPayload) {
  return emit('weekly_review_reminder', payload);
}

// ─── monthly_financial_snapshot ─────────────────────────────────────────────────────────────
// First day of month prompt to record financial snapshot and review unit economics

export interface monthly_financial_snapshotPayload {

}

export function listenmonthly_financial_snapshot(cb: (payload: monthly_financial_snapshotPayload) => void) {
  return listen<monthly_financial_snapshotPayload>('monthly_financial_snapshot', (event) => cb(event.payload));
}

export function emitmonthly_financial_snapshot(payload: monthly_financial_snapshotPayload) {
  return emit('monthly_financial_snapshot', payload);
}

// ─── quarterly_strategy_review ─────────────────────────────────────────────────────────────
// Quarterly reminder to reassess strategic position, moat, and competitors

export interface quarterly_strategy_reviewPayload {

}

export function listenquarterly_strategy_review(cb: (payload: quarterly_strategy_reviewPayload) => void) {
  return listen<quarterly_strategy_reviewPayload>('quarterly_strategy_review', (event) => cb(event.payload));
}

export function emitquarterly_strategy_review(payload: quarterly_strategy_reviewPayload) {
  return emit('quarterly_strategy_review', payload);
}
