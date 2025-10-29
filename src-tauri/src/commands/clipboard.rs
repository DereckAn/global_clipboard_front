use crate::clipboard::{read_clipboard, write_clipboard};
use crate::db::models::{ClipboardItem, CreateClipboardItemDto, UpdateClipboardItemDto};
use crate::db::repository::ClipboardRepository;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub db_path: String,
}

#[tauri::command]
pub fn get_clipboard_items(state: State<Mutex<AppState>>) -> Result<Vec<ClipboardItem>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.get_items().map_err(|e| e.to_string())
}

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
    repo.delete_item(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_all_clipboard_items(state: State<Mutex<AppState>>) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.clear_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_clipboard_items(
    query: String,
    state: State<Mutex<AppState>>,
) -> Result<Vec<ClipboardItem>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let repo = ClipboardRepository::new(&app_state.db_path).map_err(|e| e.to_string())?;
    repo.search_items(&query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_from_clipboard() -> Result<String, String> {
    read_clipboard()
}

#[tauri::command]
pub fn write_to_clipboard(text: String) -> Result<(), String> {
    write_clipboard(&text)
}
