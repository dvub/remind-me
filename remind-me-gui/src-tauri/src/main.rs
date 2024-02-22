// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use remind::reminders::{__specta__fn__read_all_reminders, read_all_reminders};
use specta::collect_types;

fn main() {
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        collect_types![read_all_reminders::<PathBuf>],
        "../src/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
