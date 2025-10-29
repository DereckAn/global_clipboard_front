use crate::clipboard::operations::read_clipboard;
use crate::clipboard::types::{detect_content_type, get_source_app, detect_code_language};
use crate::db::models::CreateClipboardItemDto;
use crate::db::repository::ClipboardRepository;
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;
use tokio::sync::Mutex;
use tokio::time::sleep;

pub struct ClipboardMonitor {
    last_content: Arc<Mutex<String>>,
    repo_path: String,
    is_running: Arc<Mutex<bool>>,
    app_handle: tauri::AppHandle,
}

impl ClipboardMonitor {
    pub fn new(repo_path: String, app_handle: tauri::AppHandle) -> Self {
        Self {
            last_content: Arc::new(Mutex::new(String::new())),
            repo_path,
            is_running: Arc::new(Mutex::new(false)),
            app_handle,
        }
    }

    pub async fn start(&self) {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            println!("Clipboard monitor already running");
            return;
        }
        *is_running = true;
        drop(is_running);

        let last_content = self.last_content.clone();
        let repo_path = self.repo_path.clone();
        let is_running = self.is_running.clone();
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            println!("Clipboard monitor started");

            loop {
                // Check if still running
                let running = *is_running.lock().await;
                if !running {
                    println!("Clipboard monitor stopped");
                    break;
                }

                // Read current clipboard content
                if let Ok(current_content) = read_clipboard() {
                    let mut last = last_content.lock().await;

                    // If content changed and not empty
                    if !current_content.is_empty() && current_content != *last {
                        println!(
                            "New clipboard content detected: {}",
                            &current_content[..std::cmp::min(50, current_content.len())]
                        );

                        // Detect content type
                        let content_type = detect_content_type(&current_content);
                        let source_app = get_source_app();
                        let code_language = if content_type == "code" {
                            detect_code_language(&current_content)
                        } else {
                            None
                        };

                        // Save to database
                        if let Ok(repo) = ClipboardRepository::new(&repo_path) {
                            let dto = CreateClipboardItemDto {
                                content_type,
                                content_text: current_content.clone(),
                                content_metadata: None,
                                source_app,
                                code_language,
                            };

                            match repo.create_item(dto) {
                                Ok(item) => {
                                    println!("Saved clipboard item: {}", item.id);

                                    // Emit event to frontend
                                    if let Err(e) = app_handle.emit("clipboard-item-added", &item) {
                                        eprintln!(
                                            "Failed to emit clipboard-item-added event: {}",
                                            e
                                        );
                                    }
                                }
                                Err(e) => eprintln!("Failed to save clipboard item: {}", e),
                            }
                        }

                        // Update last content
                        *last = current_content;
                    }
                }

                // Wait before next check (500ms)
                sleep(Duration::from_millis(500)).await;
            }
        });
    }

    // pub async fn stop(&self) {
    //     let mut is_running = self.is_running.lock().await;
    //     *is_running = false;
    //     println!("Stopping clipboard monitor...");
    // }
}
