use std::fs;
use tauri::{AppHandle, Manager, Runtime};

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
