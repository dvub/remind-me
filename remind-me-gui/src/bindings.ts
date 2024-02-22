/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

/**
 * Attempts to read a vector of Reminders from the specified path. Returns a result containing a Vector of Reminders.
 */
export function readAllReminders(path: string) {
    return invoke()<Reminder[]>("read_all_reminders", { path })
}

/**
 * Struct to represent a reminder.
 */
export type Reminder = { name: string; description: string; frequency: number; icon: string | null }
