mod clipboard;
mod colors;
mod commands;
mod db;

use clipboard::ClipboardMonitor;
use commands::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_global_shortcut::{ ShortcutState};

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

            // NUEVO: Registrar hotkey global (Command+Shift+V en Mac, Ctrl+Shift+V en Windows/Linux)
            let app_handle = app.handle().clone();
            app.handle()
                .plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["CommandOrControl+Shift+V"])?
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
            println!("Global hotkey: CommandOrControl+Shift+V");

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
            commands::toggle_window_visibility, // NUEVO
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
