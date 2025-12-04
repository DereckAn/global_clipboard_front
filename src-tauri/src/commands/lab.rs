use crate::clipboard::image_handler::{
    copy_image_file_to_storage, detect_mime_type, ensure_thumbnail, StoredImageInfo,
};
use crate::db::models::CreateClipboardItemDto;
use crate::db::repository::ClipboardRepository;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::process::Command;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Runtime, State};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LabFeatureId {
    Screenshot,
    Ocr,
    Translator,
    PickColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabFeatureMeta {
    pub id: LabFeatureId,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub needs_download: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureState {
    pub id: LabFeatureId,
    pub installed_version: Option<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabFeatureWithState {
    pub id: LabFeatureId,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub needs_download: bool,
    pub status: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

const LAB_STATE_FILE: &str = "lab_features.json";
const FEATURE_ROOT_DIR: &str = "lab_features";

fn registry() -> Vec<LabFeatureMeta> {
    vec![
        LabFeatureMeta {
            id: LabFeatureId::Screenshot,
            title: "Screenshot capture".to_string(),
            description: "Take full or region screenshots and save to clipboard.".to_string(),
            icon: "image".to_string(),
            needs_download: false,
        },
        LabFeatureMeta {
            id: LabFeatureId::Ocr,
            title: "Image OCR".to_string(),
            description: "Extract text from images (download helper).".to_string(),
            icon: "search".to_string(),
            needs_download: true,
        },
        LabFeatureMeta {
            id: LabFeatureId::Translator,
            title: "Text Translator".to_string(),
            description: "Translate copied text to any language.".to_string(),
            icon: "text".to_string(),
            needs_download: true,
        },
        LabFeatureMeta {
            id: LabFeatureId::PickColor,
            title: "Color Picker".to_string(),
            description: "Pick screen colors and copy HEX/RGB.".to_string(),
            icon: "color".to_string(),
            needs_download: true,
        },
    ]
}

fn state_path(app_state: &AppState) -> PathBuf {
    PathBuf::from(&app_state.app_data_dir).join(LAB_STATE_FILE)
}

fn load_states(app_state: &AppState) -> Vec<FeatureState> {
    let path = state_path(app_state);
    if !path.exists() {
        return Vec::new();
    }

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_states(app_state: &AppState, states: &[FeatureState]) -> Result<(), String> {
    let path = state_path(app_state);
    let json = serde_json::to_string_pretty(states).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

fn to_feature(meta: &LabFeatureMeta, states: &[FeatureState]) -> LabFeatureWithState {
    let state = states
        .iter()
        .find(|s| s.id == meta.id)
        .cloned()
        .unwrap_or_else(|| FeatureState {
            id: meta.id.clone(),
            installed_version: None,
            enabled: false,
            progress: None,
            error: None,
        });

    LabFeatureWithState {
        id: meta.id.clone(),
        title: meta.title.clone(),
        description: meta.description.clone(),
        icon: meta.icon.clone(),
        needs_download: meta.needs_download,
        status: if state.installed_version.is_some() {
            "installed".to_string()
        } else {
            "available".to_string()
        },
        enabled: state.enabled,
        installed_version: state.installed_version.clone(),
        progress: state.progress,
        error: state.error.clone(),
    }
}

#[tauri::command]
pub fn get_lab_features(state: State<Mutex<AppState>>) -> Result<Vec<LabFeatureWithState>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let states = load_states(&app_state);
    let features = registry()
        .iter()
        .map(|meta| to_feature(meta, &states))
        .collect();
    Ok(features)
}

#[tauri::command]
pub fn install_feature(
    state: State<Mutex<AppState>>,
    id: LabFeatureId,
) -> Result<LabFeatureWithState, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let mut states = load_states(&app_state);
    let meta = registry()
        .into_iter()
        .find(|m| m.id == id)
        .ok_or_else(|| "Feature not found".to_string())?;

    match id {
        LabFeatureId::Screenshot => {
            // Built-in: no download required
            update_state_entry(&mut states, &id, Some("builtin".to_string()), true, None);
            save_states(&app_state, &states)?;
            Ok(to_feature(&meta, &states))
        }
        _ => {
            // Placeholder for future downloadable helpers
            update_state_entry(
                &mut states,
                &id,
                None,
                false,
                Some("Download flow not implemented yet".to_string()),
            );
            save_states(&app_state, &states)?;
            Err("Download flow not implemented yet".to_string())
        }
    }
}

#[tauri::command]
pub fn uninstall_feature(
    state: State<Mutex<AppState>>,
    id: LabFeatureId,
) -> Result<LabFeatureWithState, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let mut states = load_states(&app_state);
    let meta = registry()
        .into_iter()
        .find(|m| m.id == id)
        .ok_or_else(|| "Feature not found".to_string())?;

    // Remove installed payload if any
    let feature_dir = PathBuf::from(&app_state.app_data_dir)
        .join(FEATURE_ROOT_DIR)
        .join(match id {
            LabFeatureId::Screenshot => "screenshot",
            LabFeatureId::Ocr => "ocr",
            LabFeatureId::Translator => "translator",
            LabFeatureId::PickColor => "pickColor",
        });
    let _ = fs::remove_dir_all(&feature_dir);

    update_state_entry(&mut states, &id, None, false, None);
    save_states(&app_state, &states)?;
    Ok(to_feature(&meta, &states))
}

#[tauri::command]
pub fn enable_feature(
    state: State<Mutex<AppState>>,
    id: LabFeatureId,
    enabled: bool,
) -> Result<LabFeatureWithState, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let mut states = load_states(&app_state);
    let meta = registry()
        .into_iter()
        .find(|m| m.id == id)
        .ok_or_else(|| "Feature not found".to_string())?;

    if let Some(entry) = states.iter_mut().find(|s| s.id == id) {
        entry.enabled = enabled;
    } else {
        states.push(FeatureState {
            id: id.clone(),
            installed_version: None,
            enabled,
            progress: None,
            error: None,
        });
    }

    save_states(&app_state, &states)?;
    Ok(to_feature(&meta, &states))
}

fn update_state_entry(
    states: &mut Vec<FeatureState>,
    id: &LabFeatureId,
    installed_version: Option<String>,
    enabled: bool,
    error: Option<String>,
) {
    if let Some(entry) = states.iter_mut().find(|s| &s.id == id) {
        entry.installed_version = installed_version;
        entry.enabled = enabled;
        entry.error = error;
    } else {
        states.push(FeatureState {
            id: id.clone(),
            installed_version,
            enabled,
            progress: None,
            error,
        });
    }
}

#[derive(Clone, Copy)]
enum CaptureMode {
    Full,
    Region,
}

#[tauri::command]
pub fn capture_full_screenshot<R: Runtime>(
    app: AppHandle<R>,
    state: State<Mutex<AppState>>,
) -> Result<(), String> {
    capture_screenshot_internal(app, state, CaptureMode::Full)
}

#[tauri::command]
pub fn capture_region_screenshot<R: Runtime>(
    app: AppHandle<R>,
    state: State<Mutex<AppState>>,
) -> Result<(), String> {
    capture_screenshot_internal(app, state, CaptureMode::Region)
}

fn capture_screenshot_internal<R: Runtime>(
    app: AppHandle<R>,
    state: State<Mutex<AppState>>,
    mode: CaptureMode,
) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let states = load_states(&app_state);
    let screenshot_enabled = states
        .iter()
        .find(|s| s.id == LabFeatureId::Screenshot)
        .map(|s| s.enabled && s.installed_version.is_some())
        .unwrap_or(false);

    if !screenshot_enabled {
        return Err("Screenshot feature is not installed or enabled".to_string());
    }

    let images_dir = PathBuf::from(&app_state.images_dir);
    fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;

    let temp_path = PathBuf::from(&app_state.app_data_dir)
        .join(FEATURE_ROOT_DIR)
        .join("screenshot");
    fs::create_dir_all(&temp_path).map_err(|e| e.to_string())?;
    let file_path = temp_path.join(format!("capture-{}.png", Uuid::new_v4()));

    match mode {
        CaptureMode::Full => run_full_capture(&file_path)?,
        CaptureMode::Region => run_region_capture(&file_path)?,
    }

    ingest_captured_image(&app_state, &app, &file_path, &images_dir)?;
    Ok(())
}

fn ingest_captured_image<R: Runtime>(
    app_state: &AppState,
    app: &AppHandle<R>,
    file_path: &Path,
    images_dir: &Path,
) -> Result<(), String> {
    // Copy into images_dir to align with existing asset paths
    fs::create_dir_all(images_dir).map_err(|e| e.to_string())?;
    let dest = images_dir.join(
        file_path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("capture.png")),
    );
    fs::copy(file_path, &dest).map_err(|e| format!("Failed to copy capture: {e}"))?;
    let info: StoredImageInfo = copy_image_file_to_storage(&dest, images_dir)?;

    // Ensure thumbnail exists
    let thumb_path = if info.thumb_path.as_os_str().is_empty() {
        ensure_thumbnail(&info.full_path).ok()
    } else {
        Some(info.thumb_path.clone())
    };

    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "width".to_string(),
        serde_json::Value::Number(info.width.into()),
    );
    metadata.insert(
        "height".to_string(),
        serde_json::Value::Number(info.height.into()),
    );
    metadata.insert(
        "file_size".to_string(),
        serde_json::Value::Number(info.file_size.into()),
    );
    metadata.insert(
        "preview_type".to_string(),
        serde_json::Value::String("image".to_string()),
    );
    if let Some(thumb) = thumb_path {
        metadata.insert(
            "thumbnail_path".to_string(),
            serde_json::Value::String(thumb.to_string_lossy().to_string()),
        );
    } else {
        metadata.insert(
            "thumbnail_path".to_string(),
            serde_json::Value::String(info.full_path.to_string_lossy().to_string()),
        );
    }
    metadata.insert(
        "original_extension".to_string(),
        serde_json::Value::String(
            info.original_extension
                .clone()
                .unwrap_or_else(|| "png".to_string()),
        ),
    );
    if let Some(name) = info.original_name.clone() {
        metadata.insert(
            "original_name".to_string(),
            serde_json::Value::String(name),
        );
    }

    let repo =
        ClipboardRepository::new(&app_state.db_path).map_err(|e| format!("DB error: {e}"))?;
    let dto = CreateClipboardItemDto {
        content_type: "image".to_string(),
        content_text: format!("Image {}x{}", info.width, info.height),
        content_metadata: Some(serde_json::Value::Object(metadata).to_string()),
        source_app: None,
        code_language: None,
    };

    let mut item = repo.create_item(dto).map_err(|e| e.to_string())?;
    let mime_type = detect_mime_type(&info.file_name);

    item.file_url = Some(info.full_path.to_string_lossy().to_string());
    item.file_name = Some(info.file_name.clone());
    item.file_size_bytes = Some(info.file_size as i64);
    item.file_mime_type = Some(mime_type.clone());
    item.file_hash = Some(info.file_hash.clone());

    repo.update_file_info(
        &item.id,
        &info.full_path.to_string_lossy(),
        &info.file_name,
        info.file_size as i64,
        &mime_type,
        Some(&info.file_hash),
    )
    .map_err(|e| e.to_string())?;

    let _ = app.emit("clipboard-item-added", &item);
    Ok(())
}

#[cfg(target_os = "macos")]
fn run_full_capture(path: &Path) -> Result<(), String> {
    let status = Command::new("screencapture")
        .arg("-x")
        .arg(path)
        .status()
        .map_err(|e| format!("Failed to run screencapture: {e}"))?;

    if !status.success() {
        return Err("screencapture failed".to_string());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn run_region_capture(path: &Path) -> Result<(), String> {
    let status = Command::new("screencapture")
        .arg("-i")
        .arg("-x")
        .arg(path)
        .status()
        .map_err(|e| format!("Failed to run screencapture: {e}"))?;

    if !status.success() {
        return Err("screencapture failed".to_string());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_full_capture(path: &Path) -> Result<(), String> {
    // Prefer grim if available, fallback to gnome-screenshot
    let grim = Command::new("grim").arg(path).status();
    match grim {
        Ok(status) if status.success() => return Ok(()),
        Ok(_) | Err(_) => {}
    }

    let fallback = Command::new("gnome-screenshot")
        .arg("-f")
        .arg(path)
        .status()
        .map_err(|e| format!("Failed to run gnome-screenshot: {e}"))?;

    if fallback.success() {
        Ok(())
    } else {
        Err("No supported screenshot tool found (install grim or gnome-screenshot)".to_string())
    }
}

#[cfg(target_os = "linux")]
fn run_region_capture(path: &Path) -> Result<(), String> {
    // Try grim + slurp
    if let Ok(selection) = Command::new("slurp").output() {
        if selection.status.success() {
            let geometry = String::from_utf8_lossy(&selection.stdout).trim().to_string();
            if !geometry.is_empty() {
                let status = Command::new("grim")
                    .arg("-g")
                    .arg(geometry)
                    .arg(path)
                    .status();
                if let Ok(status) = status {
                    if status.success() {
                        return Ok(());
                    }
                }
            }
        }
    }

    // Fallback to gnome-screenshot area mode
    let fallback = Command::new("gnome-screenshot")
        .arg("-a")
        .arg("-f")
        .arg(path)
        .status()
        .map_err(|e| format!("Failed to run gnome-screenshot: {e}"))?;

    if fallback.success() {
        Ok(())
    } else {
        Err("No supported region capture tool found (install grim+slurp or gnome-screenshot)".to_string())
    }
}

#[cfg(target_os = "windows")]
fn run_full_capture(_path: &Path) -> Result<(), String> {
    Err("Full-screen capture not implemented on Windows yet".to_string())
}

#[cfg(target_os = "windows")]
fn run_region_capture(_path: &Path) -> Result<(), String> {
    Err("Region capture not implemented on Windows yet".to_string())
}
