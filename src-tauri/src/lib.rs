#![allow(unexpected_cfgs)]

mod cleanup;
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
        .setup(|app| {
            // ================================================================
            // CONFIGURACIÓN ESPECÍFICA POR PLATAFORMA
            // ================================================================

            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Abrir DevTools automáticamente en desarrollo
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();

            // ================================================================
            // CONFIGURACIÓN COMÚN (todas las plataformas)
            // ================================================================

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            let db_path = app_data_dir.join("clipboard.db");
            let db_path_str = db_path.to_str().unwrap().to_string();

            println!("Database path: {db_path_str}");

            app.manage(Mutex::new(AppState {
                db_path: db_path_str.clone(),
            }));

            let app_handle = app.handle().clone();
            let monitor = ClipboardMonitor::new(db_path_str.clone(), app_handle);

            tauri::async_runtime::spawn(async move {
                monitor.start().await;
            });

            // Tarea automática de limpieza (cada 24 horas)
            let cleanup_db_path = db_path_str.clone();
            let cleanup_app_data_dir = app_data_dir.clone();
            tauri::async_runtime::spawn(async move {
                use tokio::time::{sleep, Duration};

                loop {
                    // Esperar 24 horas (86400 segundos)
                    sleep(Duration::from_secs(86400)).await;

                    println!("🧹 Running daily automatic cleanup...");

                    // Leer settings desde localStorage (formato JSON)
                    let settings_path = cleanup_app_data_dir.join("settings.json");
                    let settings = std::fs::read_to_string(&settings_path)
                        .ok()
                        .and_then(|contents| serde_json::from_str::<serde_json::Value>(&contents).ok());

                    if let Ok(repo) = crate::db::repository::ClipboardRepository::new(&cleanup_db_path) {
                        let conn = &repo.conn;

                        // Limpiar items por retención de días (si está habilitado)
                        if let Some(ref settings_json) = settings {
                            if settings_json.get("retentionEnabled").and_then(|v| v.as_bool()).unwrap_or(false) {
                                let retention_days = settings_json
                                    .get("retentionDays")
                                    .and_then(|v| v.as_i64())
                                    .map(|d| d as i32);

                                match crate::cleanup::cleanup_old_items(conn, retention_days) {
                                    Ok(deleted) if deleted > 0 => {
                                        println!("🧹 Deleted {} old items (retention policy)", deleted);
                                    }
                                    Err(e) => eprintln!("❌ Failed to cleanup old items: {}", e),
                                    _ => {}
                                }
                            }

                            // Limpiar items excedentes (si está habilitado)
                            if settings_json.get("maxItemsEnabled").and_then(|v| v.as_bool()).unwrap_or(false) {
                                let max_items = settings_json
                                    .get("maxLocalItems")
                                    .and_then(|v| v.as_i64())
                                    .map(|m| m as i32);

                                match crate::cleanup::cleanup_excess_items(conn, max_items) {
                                    Ok(deleted) if deleted > 0 => {
                                        println!("🧹 Deleted {} excess items (max limit)", deleted);
                                    }
                                    Err(e) => eprintln!("❌ Failed to cleanup excess items: {}", e),
                                    _ => {}
                                }
                            }
                        }
                    }

                    println!("✅ Daily cleanup completed");
                }
            });

            // Cargar hotkey guardado o usar el predeterminado
            let default_hotkey = "CommandOrControl+Shift+V";
            let saved_hotkey = app_data_dir
                .join("settings.json")
                .exists()
                .then(|| {
                    std::fs::read_to_string(app_data_dir.join("settings.json"))
                        .ok()
                        .and_then(|contents| serde_json::from_str::<serde_json::Value>(&contents).ok())
                        .and_then(|settings| settings.get("hotkey").and_then(|v| v.as_str()).map(String::from))
                })
                .flatten()
                .unwrap_or_else(|| default_hotkey.to_string());

            println!("Registering global hotkey: {saved_hotkey}");

            let app_handle = app.handle().clone();
            match shortcuts::register_shortcut_upon_start(&app_handle, &saved_hotkey) {
                Ok(_) => {
                    println!("Clipboard manager initialized");
                    println!("Global hotkey registered:{saved_hotkey}");
                }
                Err(e) => {
                    eprintln!("Warning: Failed to register globalshortcut '{saved_hotkey}': {e}");
                    eprintln!("The application will continue without the global shortcut.");
                    eprintln!("You can try changing it in Settings.");
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(focused) => {
                if !focused {
                    #[cfg(target_os = "windows")]
                    {
                        // En Windows: delay para evitar ocultar durante drag
                        let window_clone = window.clone();
                        std::thread::spawn(move || {
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            // Verificar si la ventana sigue sin foco
                            if let Ok(is_focused) = window_clone.is_focused() {
                                if !is_focused {
                                    let _ = window_clone.hide();
                                }
                            }
                        });
                    }

                    #[cfg(not(target_os = "windows"))]
                    {
                        // macOS y Linux: ocultar inmediatamente
                        let _ = window.hide();
                    }
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_item,
            commands::create_clipboard_item,
            commands::upsert_clipboard_item,
            commands::bump_clipboard_item,
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
            commands::save_cleanup_settings,
            commands::update_global_hotkey,
            commands::get_current_shortcut,
            commands::unregister_shortcut,
            commands::get_clipboard_items_paginated,
            commands::count_clipboard_items,
            commands::search_clipboard_items_fts,
            commands::count_search_results_fts,
            commands::cleanup_old_items,
            commands::cleanup_excess_items,
            commands::get_database_size,
            commands::optimize_database,
            commands::get_database_stats,
            commands::test_cleanup_preview,
            commands::test_force_cleanup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
