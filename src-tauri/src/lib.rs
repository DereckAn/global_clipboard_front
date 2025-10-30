mod clipboard;
mod colors;
mod commands;
mod db;

use clipboard::ClipboardMonitor;
use commands::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_global_shortcut::ShortcutState;

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

            println!("Database path: {}", db_path_str);

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

            // NUEVO: Leer hotkey guardado o usar default
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

            println!("Registering global hotkey: {}", saved_hotkey);

            // Registrar hotkey global con el valor guardado
            let app_handle = app.handle().clone();
            let hotkey = saved_hotkey.clone();
            app.handle()
                .plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts([hotkey.as_str()])?
                        .with_handler(move |_app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                println!("Hotkey pressed: {:?}", shortcut);
                                if let Err(e) =
                                    commands::toggle_window_visibility(app_handle.clone())
                                {
                                    eprintln!("Failed to toggle window: {}", e);
                                }
                            }
                        })
                        .build(),
                )
                .expect("Failed to register global shortcut");

            println!("Clipboard manager initialized");
            println!("Global hotkey: {}", saved_hotkey);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_items,
            commands::get_clipboard_item,
            commands::create_clipboard_item,
            commands::update_clipboard_item,
            commands::delete_clipboard_item,
            commands::clear_all_clipboard_items,
            commands::search_clipboard_items,
            commands::read_from_clipboard,
            commands::write_to_clipboard,
            commands::convert_color_formats,
            commands::extract_domain_from_url,
            commands::fetch_link_metadata,
            commands::remove_duplicate_items,
            commands::toggle_window_visibility,
            commands::get_setting,
            commands::save_setting,
            commands::update_global_hotkey
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
