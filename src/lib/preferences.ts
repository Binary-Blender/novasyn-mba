// PREFERENCES — typed localStorage hooks
// @agicore-generated — DO NOT EDIT (regenerate via: agicore compile)

import { useState, useEffect } from 'react';

export function usePreference<T>(key: string, defaultValue: T): [T, (value: T) => void] {
  const stored = localStorage.getItem(key);
  const value: T = stored !== null ? JSON.parse(stored) as T : defaultValue;
  const setValue = (newValue: T) => {
    localStorage.setItem(key, JSON.stringify(newValue));
  };
  return [value, setValue];
}

export function usePreferenceState(key: string, defaultValue: string): [string, (v: string) => void] {
  const [value, setValue] = useState(() => localStorage.getItem(key) ?? defaultValue);
  const set = (v: string) => { localStorage.setItem(key, v); setValue(v); };
  return [value, set] as const;
}

export function getDefaultModel(): string {
  const stored = localStorage.getItem('default_model');
  return stored !== null ? JSON.parse(stored) as string : 'claude-sonnet-4-5';
}

export function setDefaultModel(value: string): void {
  localStorage.setItem('default_model', JSON.stringify(value));
}

export const useDefaultModel = () => usePreferenceState('default_model', 'claude-sonnet-4-5');

export function getTheme(): string {
  const stored = localStorage.getItem('theme');
  return stored !== null ? JSON.parse(stored) as string : 'dark';
}

export function setTheme(value: string): void {
  localStorage.setItem('theme', JSON.stringify(value));
}

export const useTheme = () => usePreferenceState('theme', 'dark');

export function getWeeklyReviewReminder(): boolean {
  const stored = localStorage.getItem('weekly_review_reminder_enabled');
  return stored !== null ? JSON.parse(stored) as boolean : 'true';
}

export function setWeeklyReviewReminder(value: boolean): void {
  localStorage.setItem('weekly_review_reminder_enabled', JSON.stringify(value));
}

export const useWeeklyReviewReminder = () => usePreferenceState('weekly_review_reminder_enabled', 'true');

export function getCeoTimeTargetHours(): number {
  const stored = localStorage.getItem('ceo_time_target_hours');
  return stored !== null ? JSON.parse(stored) as number : '20';
}

export function setCeoTimeTargetHours(value: number): void {
  localStorage.setItem('ceo_time_target_hours', JSON.stringify(value));
}

export const useCeoTimeTargetHours = () => usePreferenceState('ceo_time_target_hours', '20');
