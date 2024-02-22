// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use remind::{
    get_path,
    reminders::{__cmd__read_all_reminders, read_all_reminders},
    run,
};
fn main() {
    tauri::Builder::default()
        .setup(|_| {
            println!("starting backend...");
            let _path = get_path()?;
            // TODO:
            // concurrent so this doesn't block
            //run(&path)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_all_reminders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
