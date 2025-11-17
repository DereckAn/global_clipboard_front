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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};
    use std::fs;
    use tempfile::TempDir;

    fn create_test_image(dir: &TempDir, name: &str) -> PathBuf {
        let path = dir.path().join(name);
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_fn(8, 8, |_x, _y| Rgba([255, 0, 0, 255]));
        buffer.save(&path).expect("failed to save png");
        path
    }

    #[test]
    fn ensure_thumbnail_generates_neighbor_file() {
        let dir = TempDir::new().unwrap();
        let image_path = create_test_image(&dir, "sample.png");

        let thumb_path = ensure_thumbnail(image_path.to_string_lossy().to_string()).unwrap();
        assert!(thumb_path.ends_with("_thumb.png"));
        assert!(PathBuf::from(&thumb_path).exists());

        let thumb_metadata = fs::metadata(&thumb_path).unwrap();
        assert!(thumb_metadata.len() > 0);
    }

    #[test]
    fn ensure_thumbnail_errors_when_missing_file() {
        let missing = "/tmp/this-file-should-not-exist.png";
        let result = ensure_thumbnail(missing.to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Image file not found"));
    }
}
