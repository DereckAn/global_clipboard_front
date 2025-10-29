mod clipboard;
mod colors;
mod commands;
mod db;

use clipboard::ClipboardMonitor;
use commands::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
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

            println!("Clipboard manager initialized");

            Ok(())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
