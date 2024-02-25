// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use remind::{commands::*, config::*, reminders::commands::*, run};
use specta::collect_types;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    // when built, specta generates types
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        collect_types![
            read_all_reminders,
            get_path,
            edit_reminder,
            delete_reminder,
            add_reminder
        ],
        "../src/bindings.ts",
    )
    .unwrap();

    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(String::from("quit"), "Quit"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(String::from("open"), "Open"));
    let tray = SystemTray::new().with_menu(menu);

    let tray_event_handler = |app: &AppHandle, event| {
        if let SystemTrayEvent::MenuItemClick { id, .. } = event {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            }
        }
    };
    let config = read_config(&get_config_path().unwrap()).unwrap();

    tauri::Builder::default()
        // register plugins
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None, // autostart args go here, don't think i need anything for now
        ))
        // set up the commands that will be invoked from the frontend
        .invoke_handler(tauri::generate_handler![
            read_all_reminders,
            get_path,
            edit_reminder,
            delete_reminder,
            add_reminder
        ])
        // prevents the GUI from fully closing
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        // register and configure tray/events
        .system_tray(tray)
        .on_system_tray_event(tray_event_handler)
        // run backend when GUI starts
        .setup(move |_| {
            if config.run_backend_on_gui_start {
                thread::spawn(|| {
                    run(get_path().expect("error getting path")).expect("error running backend")
                });
            }
            Ok(())
        })
        // build
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        // start minimized
        .run(move |app, event| {
            if let tauri::RunEvent::Ready = event {
                if config.start_minimized {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
            }
        });
}
