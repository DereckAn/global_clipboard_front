mod clipboard;
mod colors;
mod commands;
mod db;
mod shortcuts;

use clipboard::ClipboardMonitor;
use commands::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Ocultar del Dock en macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            // Create directory if it doesn't exist
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            // Database path
            let db_path = app_data_dir.join("clipboard.db");
            let db_path_str = db_path.to_str().unwrap().to_string();

            println!("Database path: {db_path_str}");

            // Initialize app state
            app.manage(Mutex::new(AppState {
                db_path: db_path_str.clone(),
            }));

            // Start clipboard monitor with app handle
            let app_handle = app.handle().clone();
            let monitor = ClipboardMonitor::new(db_path_str, app_handle);

            tauri::async_runtime::spawn(async move {
                monitor.start().await;
            });

            // Read saved hotkey or use default
            let saved_hotkey = match app.path().app_data_dir() {
                Ok(app_data_dir) => {
                    let settings_file = app_data_dir.join("settings.json");
                    if settings_file.exists() {
                        match std::fs::read_to_string(&settings_file) {
                            Ok(contents) => {
                                match serde_json::from_str::<serde_json::Value>(&contents) {
                                    Ok(settings) => settings
                                        .get("hotkey")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("CommandOrControl+Shift+V")
                                        .to_string(),
                                    Err(_) => "CommandOrControl+Shift+V".to_string(),
                                }
                            }
                            Err(_) => "CommandOrControl+Shift+V".to_string(),
                        }
                    } else {
                        "CommandOrControl+Shift+V".to_string()
                    }
                }
                Err(_) => "CommandOrControl+Shift+V".to_string(),
            };

            println!("Registering global hotkey: {saved_hotkey}");

            // Register initial shortcut using the helper function
            let app_handle = app.handle().clone();
            match shortcuts::register_shortcut_upon_start(&app_handle, &saved_hotkey) {
                Ok(_) => {
                    println!("Clipboard manager initialized");
                    println!("Global hotkey registered: {saved_hotkey}");
                }
                Err(e) => {
                    eprintln!("Warning: Failed to register global shortcut '{saved_hotkey}': {e}");
                    eprintln!("The application will continue without the global shortcut.");
                    eprintln!("You can try changing it in Settings.");
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    window.hide().unwrap();
                    api.prevent_close();
                }
                tauri::WindowEvent::Focused(focused) => {
                    // Ocultar cuando pierde el foco
                    if !focused {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_item,
            commands::create_clipboard_item,
            commands::update_clipboard_item,
            commands::delete_clipboard_item,
            commands::clear_all_clipboard_items,
            commands::read_from_clipboard,
            commands::write_to_clipboard,
            commands::convert_color_formats,
            commands::extract_domain_from_url,
            commands::fetch_link_metadata,
            commands::remove_duplicate_items,
            commands::toggle_window_visibility,
            commands::get_setting,
            commands::save_setting,
            commands::update_global_hotkey,
            commands::get_current_shortcut,
            commands::unregister_shortcut,
            commands::get_clipboard_items_paginated,
            commands::count_clipboard_items,
            commands::search_clipboard_items_paginated,
            commands::count_search_results,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
