use crate::cleanup;
use crate::AppState;
use std::{fs, sync::Mutex};
use tauri::{AppHandle, Manager, Runtime, State};

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
