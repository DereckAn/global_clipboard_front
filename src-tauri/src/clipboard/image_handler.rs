use arboard::ImageData;
use image::image_dimensions;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::clipboard::file_handler::fingerprint_external_file;

#[derive(Debug, Clone)]
pub struct StoredImageInfo {
    pub full_path: PathBuf,
    pub thumb_path: PathBuf,
    pub file_name: String,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub file_hash: String,
    pub original_extension: Option<String>,
    pub original_name: Option<String>,
    pub is_screenshot: bool,
}

#[cfg(target_os = "macos")]
fn read_dimensions_with_sips(path: &Path) -> Option<(u32, u32)> {
    let output = Command::new("sips")
        .arg("-g")
        .arg("pixelWidth")
        .arg("-g")
        .arg("pixelHeight")
        .arg(path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut width: Option<u32> = None;
    let mut height: Option<u32> = None;

    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix("pixelWidth:") {
            width = value.trim().parse().ok();
        } else if let Some(value) = line.strip_prefix("pixelHeight:") {
            height = value.trim().parse().ok();
        }
    }

    match (width, height) {
        (Some(w), Some(h)) => Some((w, h)),
        _ => None,
    }
}

pub fn save_image_to_disk(
    image_data: &ImageData,
    images_dir: &Path,
    is_screenshot: bool,
) -> Result<StoredImageInfo, String> {
    let image_id = Uuid::new_v4();
    let file_name = format!("{}.png", image_id);
    let thumb_name = format!("{}_thumb.png", image_id);

    let full_path = images_dir.join(&file_name);
    let thumb_path = images_dir.join(&thumb_name);

    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        image_data.bytes.to_vec(),
    )
    .ok_or("Failed to create image buffer")?;

    let dynamic_img = DynamicImage::ImageRgba8(img);
    dynamic_img
        .save(&full_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    let file_hash = calculate_file_hash(&full_path)?;

    let thumbnail = dynamic_img.resize(256, 256, imageops::FilterType::Lanczos3);
    thumbnail
        .save(&thumb_path)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    let file_size = fs::metadata(&full_path).map(|m| m.len()).unwrap_or(0);

    let mut info = StoredImageInfo {
        full_path,
        thumb_path,
        file_name,
        width: image_data.width as u32,
        height: image_data.height as u32,
        file_size,
        file_hash,
        original_extension: Some("png".to_string()),
        original_name: None,
        is_screenshot,
    };

    info.is_screenshot |= guess_screenshot_from_dimensions(info.width, info.height);

    Ok(info)
}

pub fn copy_image_file_to_storage(
    source_path: &Path,
    images_dir: &Path,
) -> Result<StoredImageInfo, String> {
    // Pointer mode (all platforms): store the ORIGINAL path, never copy.
    let _ = images_dir;

    if !source_path.exists() {
        return Err(format!(
            "Source file does not exist: {}",
            source_path.display()
        ));
    }

    let original_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .to_string();

    let original_extension = source_path
        .extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.to_lowercase());

    let (width, height) = read_external_image_dimensions(source_path).unwrap_or((0, 0));

    Ok(StoredImageInfo {
        full_path: source_path.to_path_buf(),
        thumb_path: PathBuf::new(),
        file_name: source_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("image")
            .to_string(),
        width,
        height,
        file_size: fs::metadata(source_path).map(|m| m.len()).unwrap_or(0),
        file_hash: compute_external_hash(source_path)?,
        original_extension,
        original_name: Some(original_name.clone()),
        is_screenshot: looks_like_screenshot_name(&original_name),
    })
}

fn read_external_image_dimensions(path: &Path) -> Option<(u32, u32)> {
    if let Ok((width, height)) = image_dimensions(path) {
        return Some((width, height));
    }

    // sips is a macOS-only tool; other platforms just give up here.
    #[cfg(target_os = "macos")]
    {
        read_dimensions_with_sips(path)
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

pub fn delete_image_from_disk(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("Failed to delete image: {}", e))?;
    }

    if let Some(parent) = path.parent() {
        if let Some(stem) = path.file_stem() {
            let thumb_path = parent.join(format!("{}_thumb.png", stem.to_string_lossy()));
            if thumb_path.exists() {
                fs::remove_file(thumb_path)
                    .map_err(|e| format!("Failed to delete thumbnail: {}", e))?;
            }
        }
    }

    Ok(())
}

/// Delete the on-disk assets for an image history item.
/// - Screenshots we captured (`source == "clipboard"`): delete our file + thumbnail.
/// - Image files the user copied (pointer): keep the original; delete only the thumbnail.
pub fn delete_image_assets(file_path: &str, metadata_json: &str) -> Result<(), String> {
    let meta = serde_json::from_str::<Value>(metadata_json).ok();
    let source = meta
        .as_ref()
        .and_then(|v| v.get("source"))
        .and_then(|s| s.as_str());

    if source == Some("clipboard") {
        // We captured this image — safe to delete our file and its sibling thumbnail.
        return delete_image_from_disk(file_path);
    }

    // Pointer to the user's original — never delete it. Only the generated thumbnail.
    if let Some(thumb) = meta
        .as_ref()
        .and_then(|v| v.get("thumbnail_path"))
        .and_then(|t| t.as_str())
    {
        let thumb_path = Path::new(thumb);
        if thumb_path.exists() {
            fs::remove_file(thumb_path)
                .map_err(|e| format!("Failed to delete thumbnail: {}", e))?;
        }
    }
    Ok(())
}

pub fn detect_mime_type(file_path: &str) -> String {
    let path_lower = file_path.to_lowercase();

    if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else if path_lower.ends_with(".png") {
        "image/png".to_string()
    } else if path_lower.ends_with(".gif") {
        "image/gif".to_string()
    } else if path_lower.ends_with(".webp") {
        "image/webp".to_string()
    } else if path_lower.ends_with(".bmp") {
        "image/bmp".to_string()
    } else if path_lower.ends_with(".tiff") || path_lower.ends_with(".tif") {
        "image/tiff".to_string()
    } else if path_lower.ends_with(".svg") {
        "image/svg+xml".to_string()
    } else if path_lower.ends_with(".ico") {
        "image/x-icon".to_string()
    } else if path_lower.ends_with(".heic") || path_lower.ends_with(".heif") {
        "image/heic".to_string()
    } else {
        "image/png".to_string()
    }
}

pub fn calculate_file_hash(file_path: &Path) -> Result<String, String> {
    let mut file =
        fs::File::open(file_path).map_err(|e| format!("Failed to open file for hashing: {}", e))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read file for hashing: {}", e))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn looks_like_screenshot_name(name: &str) -> bool {
    let lowered = name.to_lowercase();
    let markers = [
        "screen shot",
        "screenshot",
        "captura de pantalla",
        "スクリーンショット",
        "截圖",
    ];

    markers.iter().any(|marker| lowered.contains(marker))
}

fn guess_screenshot_from_dimensions(width: u32, height: u32) -> bool {
    if width == 0 || height == 0 {
        return false;
    }

    let area = (width as u64) * (height as u64);
    if area < (1024 * 768) as u64 {
        return false;
    }

    let (max_side, min_side) = if width > height {
        (width as f64, height as f64)
    } else {
        (height as f64, width as f64)
    };

    let aspect_ratio = max_side / min_side;
    aspect_ratio >= 1.2 && aspect_ratio <= 3.6
}

pub fn ensure_thumbnail(file_path: &Path) -> Result<PathBuf, String> {
    if !file_path.exists() {
        return Err(format!(
            "Image file not found when ensuring thumbnail: {}",
            file_path.display()
        ));
    }

    let parent = file_path
        .parent()
        .ok_or_else(|| "Image file has no parent directory".to_string())?;

    let stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid image file name".to_string())?;

    let thumb_path = parent.join(format!("{}_thumb.png", stem));

    if thumb_path.exists() {
        return Ok(thumb_path);
    }

    let img = image::open(file_path)
        .map_err(|e| format!("Failed to open image for thumbnail regeneration: {}", e))?;

    let thumbnail = img.resize(256, 256, imageops::FilterType::Lanczos3);
    thumbnail
        .save(&thumb_path)
        .map_err(|e| format!("Failed to save regenerated thumbnail: {}", e))?;

    Ok(thumb_path)
}

fn compute_external_hash(path: &Path) -> Result<String, String> {
    fingerprint_external_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn detect_mime_type_handles_common_extensions() {
        assert_eq!(detect_mime_type("photo.JPG"), "image/jpeg");
        assert_eq!(detect_mime_type("icon.svg"), "image/svg+xml");
        assert_eq!(detect_mime_type("picture.heic"), "image/heic");
        assert_eq!(detect_mime_type("drawing.bmp"), "image/bmp");
        assert_eq!(detect_mime_type("unknown.xyz"), "image/png"); // fallback
    }

    #[test]
    fn looks_like_screenshot_name_detects_languages() {
        assert!(looks_like_screenshot_name(
            "Screenshot 2024-09-01 at 10.18.00"
        ));
        assert!(looks_like_screenshot_name("Captura de pantalla 2024-01-01"));
        assert!(looks_like_screenshot_name("スクリーンショット-2024"));
        assert!(!looks_like_screenshot_name("HolidayPhoto"));
    }

    #[test]
    fn guess_screenshot_from_dimensions_filters_non_screenshots() {
        assert!(guess_screenshot_from_dimensions(4032, 3024)); // typical mac screenshot
        assert!(!guess_screenshot_from_dimensions(800, 600)); // area too small
        assert!(!guess_screenshot_from_dimensions(6000, 300)); // aspect ratio extreme
        assert!(!guess_screenshot_from_dimensions(0, 0)); // invalid
    }

    #[test]
    fn calculate_file_hash_matches_known_value() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"hello world").unwrap();
        let hash = calculate_file_hash(file.path()).unwrap();
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn delete_image_assets_removes_file_when_source_is_clipboard() {
        // Arrange: a file we "own" (a captured screenshot).
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let metadata = r#"{"source":"clipboard"}"#;

        // Act
        delete_image_assets(path.to_str().unwrap(), metadata).unwrap();

        // Assert: ours → deleted.
        assert!(!path.exists());
    }

    #[test]
    fn delete_image_assets_keeps_original_when_source_is_file() {
        // Arrange: a pointer to the user's own file.
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let metadata = r#"{"source":"file"}"#;

        // Act
        delete_image_assets(path.to_str().unwrap(), metadata).unwrap();

        // Assert: the user's original must survive.
        assert!(path.exists());
    }

    #[test]
    fn delete_image_assets_keeps_file_when_source_missing() {
        // Arrange: unknown source (garbage/empty metadata).
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        // Act
        delete_image_assets(path.to_str().unwrap(), "{}").unwrap();

        // Assert: fail-safe → never delete when unsure.
        assert!(path.exists());
    }
}
