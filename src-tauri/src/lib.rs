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

  // ============================================================================
  // CONFIGURACIÓN ESPECÍFICA DE macOS
  // ============================================================================

  #[cfg(target_os = "macos")]
  #[macro_use]
  extern crate objc;

  #[cfg(target_os = "macos")]
  use cocoa::appkit::NSWindowButton;
  #[cfg(target_os = "macos")]
  use cocoa::base::id;

  /// Ocultar botones de semáforo en macOS
  #[cfg(target_os = "macos")]
  fn setup_macos_window(window: &tauri::WebviewWindow) {
      unsafe {
          let ns_window = window.ns_window().unwrap() as id;

          // Ocultar botones de semáforo (rojo, amarillo, verde)
          let close_button: id =
              msg_send![ns_window, standardWindowButton:
  NSWindowButton::NSWindowCloseButton];
          let miniaturize_button: id =
              msg_send![ns_window, standardWindowButton:
  NSWindowButton::NSWindowMiniaturizeButton];
          let zoom_button: id =
              msg_send![ns_window, standardWindowButton:
  NSWindowButton::NSWindowZoomButton];

          if !close_button.is_null() {
              let _: () = msg_send![close_button, setHidden: 1];
          }
          if !miniaturize_button.is_null() {
              let _: () = msg_send![miniaturize_button, setHidden:
  1];
          }
          if !zoom_button.is_null() {
              let _: () = msg_send![zoom_button, setHidden: 1];
          }

          // Hacer la barra de título transparente
          let _: () = msg_send![ns_window,
  setTitlebarAppearsTransparent: 1];
          let _: () = msg_send![ns_window, setTitleVisibility: 1];
      }
  }

  // ============================================================================
  // CONFIGURACIÓN ESPECÍFICA DE WINDOWS
  // ============================================================================

  #[cfg(target_os = "windows")]
  fn setup_windows_window(window: &tauri::WebviewWindow) {
      use windows::Win32::Foundation::HWND;
      use windows::Win32::Graphics::Dwm::*;
      use windows::Win32::UI::WindowsAndMessaging::*;

      unsafe {
          let hwnd = HWND(window.hwnd().unwrap().0 as isize);

          // Ocultar botones de la barra de título (minimizar, maximizar, cerrar)
          let style = GetWindowLongW(hwnd, GWL_STYLE);
          SetWindowLongW(hwnd, GWL_STYLE, style & !(WS_SYSMENU.0 as
  i32));

          // Habilitar bordes redondeados en Windows 11
          let corner_preference = DWM_WINDOW_CORNER_PREFERENCE(2); // DWMWCP_ROUND
          let _ = DwmSetWindowAttribute(
              hwnd,
              DWMWA_WINDOW_CORNER_PREFERENCE,
              &corner_preference as *const _ as *const _,
              std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as
  u32,
          );

          // Extender el marco en el área del cliente para permitir barra de título custom
          let margins = MARGINS {
              cxLeftWidth: 0,
              cxRightWidth: 0,
              cyTopHeight: 1, // 1px para mantener funcionalidad de drag
              cyBottomHeight: 0,
          };
          let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
      }
  }

  // ============================================================================
  // FUNCIÓN PRINCIPAL
  // ============================================================================

  #[cfg_attr(mobile, tauri::mobile_entry_point)]
  pub fn run() {
      tauri::Builder::default()
          .plugin(tauri_plugin_opener::init())
          .plugin(tauri_plugin_notification::init())
          .setup(|app| {
              // ================================================================            // CONFIGURACIÓN ESPECÍFICA POR PLATAFORMA
              // ================================================================

              let window = app.get_webview_window("main").unwrap();

              #[cfg(target_os = "macos")]
              {

  app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                  setup_macos_window(&window);
              }

              #[cfg(target_os = "windows")]
              setup_windows_window(&window);

              // Abrir DevTools automáticamente en desarrollo
              #[cfg(debug_assertions)]
              window.open_devtools();

              //  ================================================================
              // CONFIGURACIÓN COMÚN (todas las plataformas)
              //  ================================================================

              let app_data_dir = app
                  .path()
                  .app_data_dir()
                  .expect("Failed to get app data dir");

              std::fs::create_dir_all(&app_data_dir)
                  .expect("Failed to create app data dir");

              let db_path = app_data_dir.join("clipboard.db");
              let db_path_str =
  db_path.to_str().unwrap().to_string();

              println!("Database path: {db_path_str}");

              app.manage(Mutex::new(AppState {
                  db_path: db_path_str.clone(),
              }));

              let app_handle = app.handle().clone();
              let monitor = ClipboardMonitor::new(db_path_str,
  app_handle);

              tauri::async_runtime::spawn(async move {
                  monitor.start().await;
              });

              let saved_hotkey = match app.path().app_data_dir() {
                  Ok(app_data_dir) => {
                      let settings_file =
  app_data_dir.join("settings.json");
                      if settings_file.exists() {
                          match
  std::fs::read_to_string(&settings_file) {
                              Ok(contents) => {
                                  match
  serde_json::from_str::<serde_json::Value>(&contents) {
                                      Ok(settings) => settings
                                          .get("hotkey")
                                          .and_then(|v| v.as_str())

  .unwrap_or("CommandOrControl+Shift+V")
                                          .to_string(),
                                      Err(_) =>
  "CommandOrControl+Shift+V".to_string(),
                                  }
                              }
                              Err(_) =>
  "CommandOrControl+Shift+V".to_string(),
                          }
                      } else {
                          "CommandOrControl+Shift+V".to_string()
                      }
                  }
                  Err(_) => "CommandOrControl+Shift+V".to_string(),
              };

              println!("Registering global hotkey: {saved_hotkey}");

              let app_handle = app.handle().clone();
              match
  shortcuts::register_shortcut_upon_start(&app_handle, &saved_hotkey)
   {
                  Ok(_) => {
                      println!("Clipboard manager initialized");
                      println!("Global hotkey registered: 
  {saved_hotkey}");
                  }
                  Err(e) => {
                      eprintln!(
                          "Warning: Failed to register global 
  shortcut '{saved_hotkey}': {e}"
                      );
                      eprintln!("The application will continue 
  without the global shortcut.");
                      eprintln!("You can try changing it in 
  Settings.");
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
                      let _ = window.hide();
                  }
              }
              _ => {}
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
              commands::search_clipboard_items_fts,
              commands::count_search_results_fts,
              commands::cleanup_old_items,
              commands::cleanup_excess_items,
              commands::get_database_size,
              commands::optimize_database,
              commands::get_database_stats,
          ])
          .run(tauri::generate_context!())
          .expect("error while running tauri application");
  }