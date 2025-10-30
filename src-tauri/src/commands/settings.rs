use std::fs;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn get_setting(app: AppHandle, key: String) -> Result<String, String> {
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
        .ok_or_else(|| format!("Setting '{}' not found", key))
}

#[tauri::command]
pub fn save_setting(app: AppHandle, key: String, value: String) -> Result<(), String> {
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
    // Guardar el setting
    save_setting(app.clone(), "hotkey".to_string(), new_hotkey.clone())?;

    println!("New hotkey saved: {}", new_hotkey);
    println!("NOTE: You must restart the app for the new hotkey to take effect");

    Ok(())
}
