use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::commands::lab::{capture_screenshot_core, CaptureMode};
use crate::AppState;

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
    app.plugin(tauri_plugin_global_shortcut::Builder::new().build())
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

/// Register screenshot shortcuts
pub fn register_screenshot_shortcuts(
    app: &AppHandle,
    full_hotkey: &str,
    region_hotkey: &str,
) -> Result<(), String> {
    let manager = app.global_shortcut();

    println!("=== Registering Screenshot Hotkeys ===");
    println!("Full hotkey: '{}'", full_hotkey);
    println!("Region hotkey: '{}'", region_hotkey);

    let mut errors: Vec<String> = Vec::new();

    // Parse and register full screenshot hotkey
    if !full_hotkey.is_empty() {
        if let Err(e) = register_single_screenshot_shortcut(
            app,
            manager,
            full_hotkey,
            CaptureMode::Full,
            "full",
        ) {
            println!("WARNING: Failed to register full screenshot hotkey: {}", e);
            errors.push(format!("Full: {}", e));
        }
    }

    // Parse and register region screenshot hotkey
    if !region_hotkey.is_empty() {
        if let Err(e) = register_single_screenshot_shortcut(
            app,
            manager,
            region_hotkey,
            CaptureMode::Region,
            "region",
        ) {
            println!(
                "WARNING: Failed to register region screenshot hotkey: {}",
                e
            );
            errors.push(format!("Region: {}", e));
        }
    }

    println!("=== Screenshot Hotkeys Registration Complete ===");

    // Only fail if both failed
    if errors.len() == 2 {
        return Err(errors.join("; "));
    }

    Ok(())
}

fn register_single_screenshot_shortcut(
    app: &AppHandle,
    manager: &tauri_plugin_global_shortcut::GlobalShortcut<tauri::Wry>,
    hotkey: &str,
    mode: CaptureMode,
    label: &str,
) -> Result<(), String> {
    let shortcut: Shortcut = hotkey
        .parse()
        .map_err(|e| format!("Invalid {} screenshot hotkey '{}': {}", label, hotkey, e))?;

    println!("Parsed {} shortcut: {:?}", label, shortcut);

    // Unregister if already registered
    if manager.is_registered(shortcut.clone()) {
        println!(
            "{} shortcut already registered, unregistering first...",
            label
        );
        manager
            .unregister(shortcut.clone())
            .map_err(|e| format!("Failed to unregister {} shortcut: {}", label, e))?;
    }

    // Set up the handler
    let app_handle = app.clone();
    let mode_clone = mode;
    let label_owned = label.to_string();

    manager
        .on_shortcut(shortcut.clone(), move |_app, shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                println!(">>> {} screenshot hotkey PRESSED!", label_owned);

                if let Some(state) = app_handle.try_state::<Mutex<AppState>>() {
                    if let Ok(app_state) = state.lock() {
                        if let Err(e) = capture_screenshot_core(&app_handle, &app_state, mode_clone)
                        {
                            eprintln!(">>> Failed to capture {} screenshot: {}", label_owned, e);
                        } else {
                            println!(">>> {} screenshot captured successfully!", label_owned);
                        }
                    } else {
                        eprintln!(">>> Failed to lock AppState");
                    }
                } else {
                    eprintln!(">>> Could not access AppState");
                }
            }
        })
        .map_err(|e| format!("Failed to set handler for {} shortcut: {}", label, e))?;

    // Register the shortcut
    manager
        .register(shortcut)
        .map_err(|e| format!("Failed to register {} shortcut '{}': {}", label, hotkey, e))?;

    println!("✓ {} screenshot hotkey registered: {}", label, hotkey);
    Ok(())
}

/// Unregister all screenshot shortcuts
pub fn unregister_screenshot_shortcuts(app: &AppHandle) -> Result<(), String> {
    let manager = app.global_shortcut();

    // We need to try common screenshot hotkeys since we don't track them
    // This is a simplified approach - ideally you'd store the registered shortcuts
    let common_shortcuts = [
        "Command+Shift+3",
        "Command+Shift+4",
        "Control+Shift+3",
        "Control+Shift+4",
        "Alt+Shift+3",
        "Alt+Shift+4",
    ];

    for shortcut_str in common_shortcuts {
        if let Ok(shortcut) = shortcut_str.parse::<Shortcut>() {
            if manager.is_registered(shortcut) {
                let _ = manager.unregister(shortcut);
            }
        }
    }

    println!("Unregistered screenshot hotkeys");
    Ok(())
}
