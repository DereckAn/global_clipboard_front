use tauri::{AppHandle, Manager};
use crate::shortcuts::{register_screenshot_shortcuts, unregister_screenshot_shortcuts};

#[tauri::command]
pub fn toggle_window_visibility(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found".to_string())?;

    if window.is_visible().map_err(|e| e.to_string())? {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_screenshot_hotkeys(
    app: AppHandle,
    full_hotkey: String,
    region_hotkey: String,
) -> Result<(), String> {
    register_screenshot_shortcuts(&app, &full_hotkey, &region_hotkey)
}

#[tauri::command]
pub async fn unregister_screenshot_hotkeys(app: AppHandle) -> Result<(), String> {
    unregister_screenshot_shortcuts(&app)
}