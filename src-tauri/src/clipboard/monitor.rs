use crate::clipboard::image_handler::{
    copy_image_file_to_storage, detect_mime_type, save_image_to_disk,
};
use crate::clipboard::listener::ClipboardEvent;
use crate::clipboard::state;
use crate::clipboard::operations::{read_clipboard_content, ClipboardContent};
use crate::clipboard::types::{detect_code_language, detect_content_type, get_source_app};
use crate::db::models::CreateClipboardItemDto;
use crate::db::repository::ClipboardRepository;
use std::sync::Arc;
use tauri::{Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;

pub struct ClipboardMonitor {
    event_rx: Mutex<Option<UnboundedReceiver<ClipboardEvent>>>,
    last_text_content: Arc<Mutex<String>>,
    last_image_hash: Arc<Mutex<Option<Vec<u8>>>>,
    last_file_path: Arc<Mutex<Option<std::path::PathBuf>>>,
    repo_path: String,
    images_dir: std::path::PathBuf,
    is_running: Arc<Mutex<bool>>,
    app_handle: tauri::AppHandle,
}

impl ClipboardMonitor {
    pub fn new(
        repo_path: String,
        app_handle: tauri::AppHandle,
        event_rx: UnboundedReceiver<ClipboardEvent>,
    ) -> Self {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir");
        let images_dir = app_data_dir.join("images");

        Self {
            event_rx: Mutex::new(Some(event_rx)),
            last_text_content: Arc::new(Mutex::new(String::new())),
            last_image_hash: Arc::new(Mutex::new(None)),
            last_file_path: Arc::new(Mutex::new(None)),
            repo_path,
            images_dir,
            is_running: Arc::new(Mutex::new(false)),
            app_handle,
        }
    }

    pub async fn start(&self) {
        {
            let mut running = self.is_running.lock().await;
            if *running {
                println!("Clipboard monitor already running");
                return;
            }
            *running = true;
        }

        let mut event_rx = self
            .event_rx
            .lock()
            .await
            .take()
            .expect("ClipboardMonitor::start called more than once");

        println!("Clipboard monitor started");

        while let Some(event) = event_rx.recv().await {
            match event {
                ClipboardEvent::Changed => self.handle_clipboard_change().await,
                ClipboardEvent::Error(err) => eprintln!("Clipboard listener error: {err}"),
            }
        }

        {
            let mut running = self.is_running.lock().await;
            *running = false;
        }

        println!("Clipboard monitor stopped");
    }

    async fn handle_clipboard_change(&self) {
        if state::take_skip_event() {
            return;
        }

        match read_clipboard_content() {
            Ok(ClipboardContent::ImageFile(file_path)) => {
                {
                    let mut last_path = self.last_file_path.lock().await;
                    if let Some(prev) = last_path.as_ref() {
                        if prev == &file_path {
                            return;
                        }
                    }
                    *last_path = Some(file_path.clone());
                }

                println!(
                    "📁 Image file detected from Finder: {}",
                    file_path.display()
                );

                match copy_image_file_to_storage(&file_path, &self.images_dir) {
                    Ok(info) => {
                        if let Ok(repo) = ClipboardRepository::new(&self.repo_path) {
                            if let Ok(Some(existing_item)) =
                                repo.find_by_file_hash(&info.file_hash)
                            {
                                println!(
                                    "🔁 Duplicate image detected (hash: {}), bumping existing item: {}",
                                    &info.file_hash[..12],
                                    existing_item.id
                                );

                                if let Ok(bumped_item) = repo.bump_item(&existing_item.id) {
                                    if let Err(e) =
                                        self.app_handle.emit("clipboard-item-added", &bumped_item)
                                    {
                                        eprintln!("Failed to emit clipboard-item-added event: {e}");
                                    }
                                }

                                let _ = std::fs::remove_file(&info.full_path);
                                let _ = std::fs::remove_file(&info.thumb_path);
                                return;
                            }
                        }

                        let mut metadata = serde_json::json!({
                            "width": info.width,
                            "height": info.height,
                            "thumbnail_path": info.thumb_path.to_string_lossy().to_string(),
                            "original_path": file_path.to_string_lossy().to_string(),
                            "is_screenshot": info.is_screenshot,
                            "source": "file",
                        });

                        if let Some(ext) = info.original_extension.as_ref() {
                            if let Some(obj) = metadata.as_object_mut() {
                                obj.insert(
                                    "original_extension".to_string(),
                                    serde_json::Value::String(ext.clone()),
                                );
                            }
                        }

                        if let Ok(repo) = ClipboardRepository::new(&self.repo_path) {
                            let dto = CreateClipboardItemDto {
                                content_type: "image".to_string(),
                                content_text: format!("Image {}x{}", info.width, info.height),
                                content_metadata: Some(metadata.to_string()),
                                source_app: get_source_app(),
                                code_language: None,
                            };

                            match repo.create_item(dto) {
                                Ok(mut item) => {
                                    let mime_type = detect_mime_type(&info.file_name);

                                    item.file_url =
                                        Some(info.full_path.to_string_lossy().to_string());
                                    item.file_name = Some(info.file_name.clone());
                                    item.file_size_bytes = Some(info.file_size as i64);
                                    item.file_mime_type = Some(mime_type.clone());
                                    item.file_hash = Some(info.file_hash.clone());

                                    if let Err(e) = repo.update_file_info(
                                        &item.id,
                                        &info.full_path.to_string_lossy(),
                                        &info.file_name,
                                        info.file_size as i64,
                                        &mime_type,
                                        Some(&info.file_hash),
                                    ) {
                                        eprintln!("Failed to update file info: {}", e);
                                    }

                                    println!("✅ Saved image from Finder: {}", item.id);

                                    if let Err(e) =
                                        self.app_handle.emit("clipboard-item-added", &item)
                                    {
                                        eprintln!("Failed to emit clipboard-item-added event: {e}");
                                    }
                                }
                                Err(e) => eprintln!("Failed to save image from Finder: {e}"),
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to copy image file: {}", e);
                    }
                }
            }
            Ok(ClipboardContent::Text(current_text)) => {
                let mut last_text = self.last_text_content.lock().await;

                if !current_text.is_empty() && current_text != *last_text {
                    println!(
                        "New clipboard text detected: {}",
                        &current_text[..std::cmp::min(50, current_text.len())]
                    );

                    let content_type = detect_content_type(&current_text);
                    let source_app = get_source_app();
                    let code_language = if content_type == "code" {
                        detect_code_language(&current_text)
                    } else {
                        None
                    };

                    if let Ok(repo) = ClipboardRepository::new(&self.repo_path) {
                        let dto = CreateClipboardItemDto {
                            content_type,
                            content_text: current_text.clone(),
                            content_metadata: None,
                            source_app,
                            code_language,
                        };

                        match repo.upsert_item(dto) {
                            Ok(item) => {
                                println!("Upserted clipboard item: {}", item.id);

                                if let Err(e) = self.app_handle.emit("clipboard-item-added", &item)
                                {
                                    eprintln!("Failed to emit clipboard-item-added event: {e}");
                                }
                            }
                            Err(e) => eprintln!("Failed to upsert clipboard item: {e}"),
                        }
                    }

                    *last_text = current_text;
                }
            }
            Ok(ClipboardContent::Image(image_data, screenshot_hint)) => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};

                let mut hasher = DefaultHasher::new();
                image_data.bytes.hash(&mut hasher);
                let current_hash = hasher.finish().to_le_bytes().to_vec();

                {
                    let mut last_hash = self.last_image_hash.lock().await;
                    if let Some(previous_hash) = last_hash.as_ref() {
                        if *previous_hash == current_hash {
                            return;
                        }
                    }
                    *last_hash = Some(current_hash.clone());
                }

                match save_image_to_disk(&image_data, &self.images_dir, screenshot_hint) {
                    Ok(info) => {
                        if let Ok(repo) = ClipboardRepository::new(&self.repo_path) {
                            if let Ok(Some(existing_item)) =
                                repo.find_by_file_hash(&info.file_hash)
                            {
                                println!(
                                    "🔁 Duplicate clipboard image detected (hash: {}), bumping existing item: {}",
                                    &info.file_hash[..12],
                                    existing_item.id
                                );

                                if let Ok(bumped_item) = repo.bump_item(&existing_item.id) {
                                    if let Err(e) =
                                        self.app_handle.emit("clipboard-item-added", &bumped_item)
                                    {
                                        eprintln!("Failed to emit clipboard-item-added event: {e}");
                                    }
                                }

                                let _ = std::fs::remove_file(&info.full_path);
                                let _ = std::fs::remove_file(&info.thumb_path);
                                return;
                            }
                        }

                        let mut metadata = serde_json::json!({
                            "width": info.width,
                            "height": info.height,
                            "thumbnail_path": info.thumb_path.to_string_lossy().to_string(),
                            "original_path": info.full_path.to_string_lossy().to_string(),
                            "is_screenshot": info.is_screenshot,
                            "source": "clipboard",
                        });

                        if let Some(ext) = info.original_extension.as_ref() {
                            if let Some(obj) = metadata.as_object_mut() {
                                obj.insert(
                                    "original_extension".to_string(),
                                    serde_json::Value::String(ext.clone()),
                                );
                            }
                        }

                        if let Ok(repo) = ClipboardRepository::new(&self.repo_path) {
                            let dto = CreateClipboardItemDto {
                                content_type: "image".to_string(),
                                content_text: format!("Image {}x{}", info.width, info.height),
                                content_metadata: Some(metadata.to_string()),
                                source_app: get_source_app(),
                                code_language: None,
                            };

                            match repo.create_item(dto) {
                                Ok(mut item) => {
                                    let mime_type = detect_mime_type(&info.file_name);

                                    item.file_url =
                                        Some(info.full_path.to_string_lossy().to_string());
                                    item.file_name = Some(info.file_name.clone());
                                    item.file_size_bytes = Some(info.file_size as i64);
                                    item.file_mime_type = Some(mime_type.clone());
                                    item.file_hash = Some(info.file_hash.clone());

                                    if let Err(e) = repo.update_file_info(
                                        &item.id,
                                        &info.full_path.to_string_lossy(),
                                        &info.file_name,
                                        info.file_size as i64,
                                        &mime_type,
                                        Some(&info.file_hash),
                                    ) {
                                        eprintln!("Failed to update file info: {}", e);
                                    }

                                    println!("Saved clipboard image: {}", item.id);

                                    if let Err(e) =
                                        self.app_handle.emit("clipboard-item-added", &item)
                                    {
                                        eprintln!(
                                            "Failed to emit clipboard-item-added event: {}",
                                            e
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to save clipboard image: {}", e)
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to save image to disk: {}", e),
                }
            }
            Ok(ClipboardContent::Empty) => {
                // No-op
            }
            Err(e) => {
                eprintln!("Failed to read clipboard: {}", e);
            }
        }
    }
}
