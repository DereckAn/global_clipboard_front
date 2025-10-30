use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// Register a shortcut with a handler for toggling window visibility
/// This is used for dynamically changing the shortcut at runtime
pub fn register_shortcut(app: &AppHandle, shortcut_str: &str) -> Result<(), String> {
    let shortcut = shortcut_str
        .parse::<Shortcut>()
        .map_err(|e| format!("Invalid shortcut format: {e}"))?;

    let shortcut_display = shortcut_str.to_string();
    let app_clone = app.clone();

    // Register the shortcut handler FIRST
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                println!("Dynamic hotkey pressed: {shortcut_display}");

                // Use the cloned app handle to get the window
                if let Some(window) = app_clone.get_webview_window("main") {
                    match window.is_visible() {
                        Ok(true) => {
                            println!("Hiding window...");
                            let _ = window.hide();
                        }
                        Ok(false) => {
                            println!("Showing window...");
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                        Err(e) => {
                            eprintln!("Failed to check window visibility: {e}");
                        }
                    }
                } else {
                    eprintln!("Could not find main window!");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    // THEN register the shortcut
    app.global_shortcut()
        .register(shortcut)
        .map_err(|e| e.to_string())?;

    println!("Registered dynamic shortcut: {shortcut_str}");
    Ok(())
}

/// Register a shortcut upon application start
/// This is used during the initial setup in lib.rs
pub fn register_shortcut_upon_start(app: &AppHandle, shortcut_str: &str) -> Result<(), String> {
    // Initialize the plugin WITHOUT any shortcuts or handlers
    // We'll register them separately using register_shortcut()
    app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .build()
    )
    .map_err(|e| e.to_string())?;

    println!("Global shortcut plugin initialized");

    // Now register the initial shortcut using the same method as dynamic changes
    register_shortcut(app, shortcut_str)?;

    Ok(())
}

/// Unregister all shortcuts
pub fn unregister_all_shortcuts(app: &AppHandle) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())?;

    println!("Unregistered all shortcuts");
    Ok(())
}
