use crate::cleanup;
use crate::AppState;
use serde::Serialize;
use std::{fs, sync::Mutex};
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_autostart::ManagerExt;

#[derive(Debug, Serialize)]
pub struct CleanupSettings {
    pub max_items_enabled: bool,
    pub max_local_items: i32,
    pub retention_enabled: bool,
    pub retention_days: i32,
}

#[tauri::command]
pub fn get_setting<R: Runtime>(app: AppHandle<R>, key: String) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    let settings_file = app_data_dir.join("settings.json");

    if !settings_file.exists() {
        return Err("Settings file not found".to_string());
    }

    let contents = fs::read_to_string(settings_file).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

    settings
        .get(&key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Setting '{key}' not found"))
}

#[tauri::command]
pub fn save_setting<R: Runtime>(
    app: AppHandle<R>,
    key: String,
    value: String,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    let settings_file = app_data_dir.join("settings.json");

    // Load existing settings or create new
    let mut settings: serde_json::Value = if settings_file.exists() {
        let contents = fs::read_to_string(&settings_file).map_err(|e| e.to_string())?;
        serde_json::from_str(&contents).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Update setting
    settings[key] = serde_json::Value::String(value);

    // Save to file
    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(settings_file, json_string).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_global_hotkey(app: AppHandle, new_hotkey: String) -> Result<(), String> {
    println!("Updating hotkey to: {new_hotkey}");

    // Save the setting first
    save_setting(app.clone(), "hotkey".to_string(), new_hotkey.clone())?;
    println!("Setting saved");

    // Unregister all existing shortcuts AND handlers
    crate::shortcuts::unregister_all_shortcuts(&app)?;
    println!("All shortcuts unregistered");

    // Small delay to ensure cleanup is complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Register the new shortcut using the helper function
    crate::shortcuts::register_shortcut(&app, &new_hotkey)?;
    println!("New shortcut registered successfully");

    Ok(())
}

#[tauri::command]
pub fn get_current_shortcut(app: AppHandle) -> Result<String, String> {
    get_setting(app, "hotkey".to_string()).or_else(|_| Ok("CommandOrControl+Shift+V".to_string()))
}

/// Save all cleanup settings at once for the background task
#[tauri::command]
pub fn save_cleanup_settings<R: Runtime>(
    app: AppHandle<R>,
    max_items_enabled: bool,
    max_local_items: i32,
    retention_enabled: bool,
    retention_days: i32,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let settings_file = app_data_dir.join("settings.json");

    // Load existing settings or create new
    let mut settings: serde_json::Value = if settings_file.exists() {
        let contents = fs::read_to_string(&settings_file).map_err(|e| e.to_string())?;
        serde_json::from_str(&contents).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Update cleanup settings
    settings["maxItemsEnabled"] = serde_json::json!(max_items_enabled);
    settings["maxLocalItems"] = serde_json::json!(max_local_items);
    settings["retentionEnabled"] = serde_json::json!(retention_enabled);
    settings["retentionDays"] = serde_json::json!(retention_days);

    // Save to file
    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(settings_file, json_string).map_err(|e| e.to_string())?;

    println!(
        "✅ Cleanup settings saved: max_enabled={}, max={}, retention_enabled={}, days={}",
        max_items_enabled, max_local_items, retention_enabled, retention_days
    );

    Ok(())
}

/// Load cleanup settings stored in `settings.json`, falling back to defaults when absent.
#[tauri::command]
pub fn get_cleanup_settings<R: Runtime>(app: AppHandle<R>) -> Result<CleanupSettings, String> {
    let defaults = CleanupSettings {
        max_items_enabled: false,
        max_local_items: 100,
        retention_enabled: false,
        retention_days: 30,
    };

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let settings_file = app_data_dir.join("settings.json");

    if !settings_file.exists() {
        return Ok(defaults);
    }

    let contents = fs::read_to_string(&settings_file).map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap_or_default();

    let max_items_enabled = json
        .get("maxItemsEnabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(defaults.max_items_enabled);
    let max_local_items = json
        .get("maxLocalItems")
        .and_then(|v| v.as_i64())
        .unwrap_or(defaults.max_local_items as i64) as i32;
    let retention_enabled = json
        .get("retentionEnabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(defaults.retention_enabled);
    let retention_days = json
        .get("retentionDays")
        .and_then(|v| v.as_i64())
        .unwrap_or(defaults.retention_days as i64) as i32;

    Ok(CleanupSettings {
        max_items_enabled,
        max_local_items,
        retention_enabled,
        retention_days,
    })
}

/// TEST COMMAND: Preview what would be deleted by cleanup (without deleting)
#[tauri::command]
pub fn test_cleanup_preview(
    state: State<Mutex<AppState>>,
    retention_days: Option<i32>,
    max_items: Option<i32>,
) -> Result<serde_json::Value, String> {
    use chrono::{Duration, Utc};

    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = crate::db::repository::ClipboardRepository::new(&app_state.db_path)
        .map_err(|e| e.to_string())?;

    let mut result = serde_json::json!({});

    // Preview retention cleanup
    if let Some(days) = retention_days {
        let cutoff_date = Utc::now() - Duration::days(days as i64);
        let cutoff_str = cutoff_date.to_rfc3339();

        let count: i64 = repo
            .conn
            .query_row(
                "SELECT COUNT(*) FROM clipboard_items WHERE updated_at < ?1 AND is_favorite = 0",
                [&cutoff_str],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        result["retention"] = serde_json::json!({
            "would_delete": count,
            "cutoff_date": cutoff_str,
            "retention_days": days
        });
    }

    // Preview excess cleanup
    if let Some(max) = max_items {
        let count: i64 = repo
            .conn
            .query_row(
                "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 0",
                [],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        let would_delete = if count > max as i64 {
            count - max as i64
        } else {
            0
        };

        result["excess"] = serde_json::json!({
            "current_count": count,
            "max_items": max,
            "would_delete": would_delete
        });
    }

    Ok(result)
}

/// TEST COMMAND: Force cleanup immediately (for testing)
#[tauri::command]
pub fn test_force_cleanup(state: State<Mutex<AppState>>) -> Result<serde_json::Value, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = crate::db::repository::ClipboardRepository::new(&app_state.db_path)
        .map_err(|e| e.to_string())?;

    // Read settings
    let app_data_dir = std::path::PathBuf::from(&app_state.db_path)
        .parent()
        .ok_or("Invalid path")?
        .to_path_buf();

    let settings_path = app_data_dir.join("settings.json");
    let settings = fs::read_to_string(&settings_path)
        .ok()
        .and_then(|contents| serde_json::from_str::<serde_json::Value>(&contents).ok());

    let mut result = serde_json::json!({});

    if let Some(ref settings_json) = settings {
        // Run retention cleanup
        if settings_json
            .get("retentionEnabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let retention_days = settings_json
                .get("retentionDays")
                .and_then(|v| v.as_i64())
                .map(|d| d as i32);

            let deleted = cleanup::cleanup_old_items(&repo.conn, retention_days)
                .map_err(|e| e.to_string())?;

            result["retention_deleted"] = serde_json::json!(deleted);
        }

        // Run excess cleanup
        if settings_json
            .get("maxItemsEnabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let max_items = settings_json
                .get("maxLocalItems")
                .and_then(|v| v.as_i64())
                .map(|m| m as i32);

            let deleted =
                cleanup::cleanup_excess_items(&repo.conn, max_items).map_err(|e| e.to_string())?;

            result["excess_deleted"] = serde_json::json!(deleted);
        }
    }

    Ok(result)
}

#[tauri::command]
pub fn unregister_shortcut(app: AppHandle) -> Result<(), String> {
    crate::shortcuts::unregister_all_shortcuts(&app)
}

#[tauri::command]
pub fn cleanup_old_items(
    retention_days: Option<i32>,
    state: State<Mutex<AppState>>,
) -> Result<usize, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(&app_state.db_path).map_err(|e| e.to_string())?;
    cleanup::cleanup_old_items(&conn, retention_days).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cleanup_excess_items(
    max_items: Option<i32>,
    state: State<Mutex<AppState>>,
) -> Result<usize, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(&app_state.db_path).map_err(|e| e.to_string())?;
    cleanup::cleanup_excess_items(&conn, max_items).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_database_size(state: State<Mutex<AppState>>) -> Result<u64, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    cleanup::get_database_size(&app_state.db_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn optimize_database(state: State<Mutex<AppState>>) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(&app_state.db_path).map_err(|e| e.to_string())?;
    cleanup::optimize_database(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_database_stats(state: State<Mutex<AppState>>) -> Result<serde_json::Value, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(&app_state.db_path).map_err(|e| e.to_string())?;

    let total_items: i64 = conn
        .query_row("SELECT COUNT(*) FROM clipboard_items", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let favorites: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let snippets: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_snippet = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let db_size = cleanup::get_database_size(&app_state.db_path).map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "total_items": total_items,
        "favorites": favorites,
        "snippets": snippets,
        "database_size_bytes": db_size,
        "database_size_mb": (db_size as f64) / (1024.0 * 1024.0),
    }))
}

// Auto start commands
#[tauri::command]
pub fn enable_autostart(app: AppHandle) -> Result<(), String> {
    let autostart = app.autolaunch();
    autostart
        .enable()
        .map_err(|e| format!("Failed to enable autostart : {}", e))?;

    println!("✅ Autostart enabled");
    Ok(())
}

#[tauri::command]
pub fn disable_autostart(app: AppHandle) -> Result<(), String> {
    let autostart = app.autolaunch();
    autostart
        .disable()
        .map_err(|e| format!("Failed to disable autostart : {}", e))?;

    println!("✅ Autostart disabled");
    Ok(())
}

#[tauri::command]
pub fn is_autostart_enabled(app: AppHandle) -> Result<bool, String> {
    let autostart = app.autolaunch();
    let enable = autostart
        .is_enabled()
        .map_err(|e| format!("Failed to check autostart status : {}", e))?;
    Ok(enable)
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    println!("🛑 Quitting application...");

    // Spawn a thread to exit after a tiny delay
    // This allows the command to return successfully to frontend
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        app.exit(0);
    });
}

#[tauri::command]
pub fn set_tray_visible(visible: bool, tray: State<Mutex<TrayIcon>>) -> Result<(), String> {
    let tray_icon = tray.lock().map_err(|e| e.to_string())?;
    if visible {
        tray_icon.set_visible(true).map_err(|e| e.to_string())?;
        println!("✅ Tray icon shown");
    } else {
        tray_icon.set_visible(false).map_err(|e| e.to_string())?;
        println!("✅ Tray icon hidden");
    }

    Ok(())
}

#[tauri::command]
pub fn is_tray_visible() -> Result<bool, String> {
    Ok(true)
}
