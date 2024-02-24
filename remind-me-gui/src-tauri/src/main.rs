// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use remind::{commands::*, reminders::commands::*};
use specta::collect_types;
use tauri_plugin_autostart::MacosLauncher;
fn main() {
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        collect_types![read_all_reminders, get_path, edit_reminder, delete_reminder],
        "../src/bindings.ts",
    )
    .unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_all_reminders,
            get_path,
            edit_reminder,
            delete_reminder
        ])
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None, // autostart args go here, don't think i need anything for now
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
