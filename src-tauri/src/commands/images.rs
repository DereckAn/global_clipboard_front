use std::path::PathBuf;

use crate::clipboard::image_handler;

/// Ensure that a thumbnail exists for the given image file. Returns the thumbnail path.
#[tauri::command]
pub fn ensure_thumbnail(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("Image file not found: {}", file_path));
    }

    let thumb = image_handler::ensure_thumbnail(&path)?;
    Ok(thumb.to_string_lossy().to_string())
}
