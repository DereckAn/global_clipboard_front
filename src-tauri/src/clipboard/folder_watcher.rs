//! Opt-in capture-folder watcher.
//!
//! Watches the user's screenshot/recording folders and, when a new image or
//! video file appears, adds a *pointer* clipboard item (no byte copy) — the
//! same shape the clipboard monitor produces for files copied from a file
//! manager. This lets screenshots live in history as references instead of
//! heavy byte copies.
//!
//! It can be started/stopped at runtime (like the global hotkeys) via the
//! `set_folder_watcher` command, and is also started at launch when the
//! `watchFoldersEnabled` setting is true. Holding the `Debouncer` alive keeps
//! it running; dropping it stops watching.

use crate::clipboard::document_thumbnail::generate_document_thumbnail;
use crate::clipboard::file_handler::{prepare_file_metadata, store_prepared_file};
use crate::clipboard::image_handler::{
    copy_image_file_to_storage, detect_mime_type, generate_image_thumbnail,
};
use crate::db::models::CreateClipboardItemDto;
use crate::db::repository::ClipboardRepository;

use notify_debouncer_mini::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

const IMAGE_EXTS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "bmp", "tiff", "heic", "heif",
];
const VIDEO_EXTS: &[&str] = &[
    "mp4", "mkv", "mov", "avi", "webm", "wmv", "m4v", "flv", "mpeg", "mpg",
];

/// Paths and storage locations the watcher needs to create pointer items.
pub struct WatcherConfig {
    pub repo_path: String,
    pub images_dir: PathBuf,
    pub files_dir: PathBuf,
    pub file_thumbs_dir: PathBuf,
    pub dirs: Vec<PathBuf>,
}

/// Live handle to the running watcher. Stored in Tauri state; dropping the
/// inner `Debouncer` stops watching (used to toggle the feature off at runtime).
#[derive(Default)]
pub struct WatcherState {
    debouncer: Option<Debouncer<RecommendedWatcher>>,
}

impl WatcherState {
    pub fn set(&mut self, debouncer: Debouncer<RecommendedWatcher>) {
        self.debouncer = Some(debouncer);
    }

    pub fn stop(&mut self) {
        self.debouncer = None; // drop → unwatch
    }
}

/// Build and start a watcher over `config.dirs`. The returned `Debouncer` must
/// be kept alive (store it in `WatcherState`) for watching to continue.
pub fn build(
    app_handle: AppHandle,
    config: WatcherConfig,
) -> Result<Debouncer<RecommendedWatcher>, String> {
    let dirs = config.dirs.clone();

    // The debouncer only forwards a path once it has stopped changing for the
    // timeout window — so we never read a half-written screenshot.
    let mut debouncer = new_debouncer(
        Duration::from_millis(750),
        move |result: DebounceEventResult| {
            if let Ok(events) = result {
                for event in events {
                    if let Err(err) = ingest(&app_handle, &config, &event.path) {
                        eprintln!("📂 Failed to ingest {}: {err}", event.path.display());
                    }
                }
            }
        },
    )
    .map_err(|e| format!("Failed to create debouncer: {e}"))?;

    let mut watching_any = false;
    for dir in &dirs {
        if dir.exists() {
            debouncer
                .watcher()
                .watch(dir, RecursiveMode::NonRecursive)
                .map_err(|e| format!("Failed to watch {}: {e}", dir.display()))?;
            println!("📂 Watching capture folder: {}", dir.display());
            watching_any = true;
        } else {
            eprintln!("📂 Capture folder missing, skipping: {}", dir.display());
        }
    }

    if !watching_any {
        return Err("no existing folders to watch".to_string());
    }

    Ok(debouncer)
}

/// Read `watchFoldersEnabled` from settings.json (stored as a string by
/// `save_setting`, so accept "true" or a real bool).
pub fn is_enabled(app_data_dir: &Path) -> bool {
    read_settings(app_data_dir)
        .get("watchFoldersEnabled")
        .map(|v| v.as_bool().unwrap_or(v.as_str() == Some("true")))
        .unwrap_or(false)
}

/// Resolve the watcher configuration from settings.json, expanding `~` and
/// falling back to `~/Pictures` and `~/Videos`.
pub fn config_from_settings(app_data_dir: &Path, repo_path: &str) -> WatcherConfig {
    let settings = read_settings(app_data_dir);
    // `HOME` on Linux/macOS, `USERPROFILE` on Windows.
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();

    let expand = |raw: &str, default_sub: &str| -> PathBuf {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return PathBuf::from(&home).join(default_sub);
        }
        if let Some(rest) = trimmed.strip_prefix("~/") {
            return PathBuf::from(&home).join(rest);
        }
        PathBuf::from(trimmed)
    };

    let screenshots = expand(
        settings
            .get("screenshotsDir")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        "Pictures",
    );
    let recordings = expand(
        settings
            .get("recordingsDir")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        "Videos",
    );

    let mut dirs = vec![screenshots];
    if !dirs.contains(&recordings) {
        dirs.push(recordings);
    }

    let files_dir = app_data_dir.join("files");
    let file_thumbs_dir = app_data_dir.join("file_thumbnails");
    let _ = std::fs::create_dir_all(&files_dir);
    let _ = std::fs::create_dir_all(&file_thumbs_dir);

    WatcherConfig {
        repo_path: repo_path.to_string(),
        images_dir: app_data_dir.join("images"),
        files_dir,
        file_thumbs_dir,
        dirs,
    }
}

fn read_settings(app_data_dir: &Path) -> serde_json::Value {
    std::fs::read_to_string(app_data_dir.join("settings.json"))
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_else(|| serde_json::json!({}))
}

fn ingest(app_handle: &AppHandle, config: &WatcherConfig, path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Ok(());
    }

    let ext = match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
    {
        Some(e) => e,
        None => return Ok(()),
    };

    if IMAGE_EXTS.contains(&ext.as_str()) {
        ingest_image(app_handle, config, path)
    } else if VIDEO_EXTS.contains(&ext.as_str()) {
        ingest_video(app_handle, config, path)
    } else {
        Ok(())
    }
}

fn ingest_image(app_handle: &AppHandle, config: &WatcherConfig, path: &Path) -> Result<(), String> {
    let repo = ClipboardRepository::new(&config.repo_path).map_err(|e| e.to_string())?;

    // Pointer-mode: stores the original path, no copy.
    let info = copy_image_file_to_storage(path, &config.images_dir)?;

    if let Ok(Some(existing)) = repo.find_by_file_hash(&info.file_hash) {
        if let Ok(bumped) = repo.bump_item(&existing.id) {
            let _ = app_handle.emit("clipboard-item-added", &bumped);
        }
        return Ok(());
    }

    // Thumbnail with the bundled image crate first (works everywhere, no
    // system thumbnailer needed); fall back to the OS thumbnailer for formats
    // the crate can't decode (e.g. HEIC).
    let thumbnail =
        generate_image_thumbnail(&info.full_path, &config.file_thumbs_dir, &info.file_hash)
            .or_else(|| {
                generate_document_thumbnail(&info.full_path, &config.file_thumbs_dir)
                    .ok()
                    .flatten()
            })
            .map(|p| p.to_string_lossy().to_string());

    let external_path = info.full_path.to_string_lossy().to_string();
    let mut metadata = serde_json::json!({
        "width": info.width,
        "height": info.height,
        "original_path": external_path,
        "external_path": external_path,
        "external_missing": false,
        "is_screenshot": info.is_screenshot,
        "source": "file",
    });
    if let (Some(obj), Some(t)) = (metadata.as_object_mut(), thumbnail) {
        obj.insert(
            "preview_type".to_string(),
            serde_json::Value::String("image".to_string()),
        );
        obj.insert("thumbnail_path".to_string(), serde_json::Value::String(t));
    }

    let dto = CreateClipboardItemDto {
        content_type: "image".to_string(),
        content_text: format!("Image {}x{}", info.width, info.height),
        content_metadata: Some(metadata.to_string()),
        source_app: None,
        code_language: None,
    };

    let mut item = repo.create_item(dto).map_err(|e| e.to_string())?;
    let mime_type = detect_mime_type(&info.file_name);
    repo.update_file_info(
        &item.id,
        &info.full_path.to_string_lossy(),
        &info.file_name,
        info.file_size as i64,
        &mime_type,
        Some(&info.file_hash),
    )
    .map_err(|e| e.to_string())?;

    item.file_url = Some(info.full_path.to_string_lossy().to_string());
    item.file_name = Some(info.file_name.clone());
    item.file_size_bytes = Some(info.file_size as i64);
    item.file_mime_type = Some(mime_type);
    item.file_hash = Some(info.file_hash.clone());

    println!("📂 Captured screenshot pointer: {}", path.display());
    let _ = app_handle.emit("clipboard-item-added", &item);
    Ok(())
}

fn ingest_video(app_handle: &AppHandle, config: &WatcherConfig, path: &Path) -> Result<(), String> {
    let repo = ClipboardRepository::new(&config.repo_path).map_err(|e| e.to_string())?;

    let prepared = prepare_file_metadata(path)?;

    if let Ok(Some(existing)) = repo.find_by_file_hash(&prepared.hash) {
        if let Ok(bumped) = repo.bump_item(&existing.id) {
            let _ = app_handle.emit("clipboard-item-added", &bumped);
        }
        return Ok(());
    }

    // Pointer-mode: stores the original path, no copy.
    let info = store_prepared_file(&prepared, path, &config.files_dir)?;

    let thumbnail = generate_document_thumbnail(&info.full_path, &config.file_thumbs_dir)
        .ok()
        .flatten()
        .map(|p| p.to_string_lossy().to_string());

    let external_path = info
        .original_path
        .as_ref()
        .unwrap_or(&info.full_path)
        .to_string_lossy()
        .to_string();
    let mut metadata = serde_json::json!({
        "source": "file",
        "original_extension": info.original_extension,
        "original_name": info.original_name,
        "external_path": external_path,
        "external_missing": false,
    });
    if let (Some(obj), Some(t)) = (metadata.as_object_mut(), thumbnail) {
        obj.insert(
            "preview_type".to_string(),
            serde_json::Value::String("image".to_string()),
        );
        obj.insert("thumbnail_path".to_string(), serde_json::Value::String(t));
    }

    let dto = CreateClipboardItemDto {
        content_type: "file".to_string(),
        content_text: info.file_name.clone(),
        content_metadata: Some(metadata.to_string()),
        source_app: None,
        code_language: None,
    };

    let mut item = repo.create_item(dto).map_err(|e| e.to_string())?;
    repo.update_file_info(
        &item.id,
        &info.full_path.to_string_lossy(),
        &info.file_name,
        info.file_size as i64,
        &info.file_mime_type,
        Some(&info.file_hash),
    )
    .map_err(|e| e.to_string())?;

    item.file_url = Some(info.full_path.to_string_lossy().to_string());
    item.file_name = Some(info.file_name.clone());
    item.file_size_bytes = Some(info.file_size as i64);
    item.file_mime_type = Some(info.file_mime_type.clone());
    item.file_hash = Some(info.file_hash.clone());

    println!("📂 Captured recording pointer: {}", path.display());
    let _ = app_handle.emit("clipboard-item-added", &item);
    Ok(())
}
