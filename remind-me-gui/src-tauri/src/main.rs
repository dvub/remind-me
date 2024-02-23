// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use remind::{
    commands::{__cmd__get_path, __specta__fn__get_path, get_path},
    reminders::{__cmd__read_all_reminders, __specta__fn__read_all_reminders, read_all_reminders},
};
use specta::collect_types;

fn main() {
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        collect_types![read_all_reminders, get_path],
        "../src/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_all_reminders, get_path])
        .plugin(tauri_plugin_fs_watch::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
