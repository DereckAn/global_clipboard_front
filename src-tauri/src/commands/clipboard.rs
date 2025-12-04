use crate::clipboard::operations::write_clipboard_image;
use crate::clipboard::{asset_cleanup, read_clipboard, write_clipboard};
use crate::db::models::{ClipboardItem, CreateClipboardItemDto, UpdateClipboardItemDto};
use crate::db::repository::ClipboardRepository;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub db_path: String,
    pub images_dir: String,
    pub app_data_dir: String,
}

// Removed: Use get_clipboard_items_paginated() instead for better performance

#[tauri::command]
pub fn get_clipboard_item(
    id: String,
    state: State<Mutex<AppState>>,
) -> Result<Option<ClipboardItem>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.get_item(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_clipboard_item(
    dto: CreateClipboardItemDto,
    state: State<Mutex<AppState>>,
) -> Result<ClipboardItem, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.create_item(dto).map_err(|e| e.to_string())
}

/// Create or update item - prevents duplicates by bumping existing items
#[tauri::command]
pub fn upsert_clipboard_item(
    dto: CreateClipboardItemDto,
    state: State<Mutex<AppState>>,
) -> Result<ClipboardItem, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.upsert_item(dto).map_err(|e| e.to_string())
}

/// Bump an existing item to the top (update timestamp)
#[tauri::command]
pub fn bump_clipboard_item(
    id: String,
    state: State<Mutex<AppState>>,
) -> Result<ClipboardItem, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.bump_item(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_clipboard_item(
    id: String,
    dto: UpdateClipboardItemDto,
    state: State<Mutex<AppState>>,
) -> Result<ClipboardItem, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.update_item(&id, dto).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_clipboard_item(id: String, state: State<Mutex<AppState>>) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    if let Some(item) = repo.get_item(&id).map_err(|e| e.to_string())? {
        if let Some(file_url) = item.file_url.as_deref() {
            asset_cleanup::delete_file_url(&item.content_type, file_url, &item.content_metadata);
        }
    }
    repo.delete_item(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_all_clipboard_items(state: State<Mutex<AppState>>) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;

    let mut stmt = repo
        .conn
        .prepare(
            "SELECT content_type, content_metadata, file_url
             FROM clipboard_items
             WHERE file_url IS NOT NULL",
        )
        .map_err(|e| e.to_string())?;
    let file_assets: Vec<(String, Option<String>, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|res| res.ok())
        .collect();

    repo.clear_all().map_err(|e| e.to_string())?;

    for (content_type, metadata, path) in file_assets {
        asset_cleanup::delete_file_url(&content_type, &path, metadata.as_deref().unwrap_or("{}"));
    }
    Ok(())
}

#[tauri::command]
pub fn read_from_clipboard() -> Result<String, String> {
    read_clipboard()
}

#[tauri::command]
pub fn write_to_clipboard(text: String) -> Result<(), String> {
    write_clipboard(&text)
}

#[tauri::command]
pub fn remove_duplicate_items(state: State<Mutex<AppState>>) -> Result<usize, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.remove_duplicates().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cleanup_missing_clipboard_files(
    state: State<Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.cleanup_missing_file_records()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_clipboard_items_paginated(
    limit: i64,
    offset: i64,
    state: State<Mutex<AppState>>,
) -> Result<Vec<ClipboardItem>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.get_items_paginated(limit, offset)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn count_clipboard_items(state: State<Mutex<AppState>>) -> Result<i64, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.count_items().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_clipboard_items_fts(
    query: String,
    limit: i64,
    offset: i64,
    state: State<Mutex<AppState>>,
) -> Result<Vec<ClipboardItem>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.search_items_fts(&query, limit, offset)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn count_search_results_fts(
    query: String,
    state: State<Mutex<AppState>>,
) -> Result<i64, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.count_search_results_fts(&query)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_image_to_clipboard(image_path: String) -> Result<(), String> {
    write_clipboard_image(&image_path)
}

#[tauri::command]
pub fn write_file_to_clipboard(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        crate::clipboard::file_handler::write_file_to_clipboard(&path)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        Err("File clipboard operations are only supported on macOS.".to_string())
    }
}
