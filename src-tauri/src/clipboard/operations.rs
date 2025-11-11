use arboard::{Clipboard, ImageData};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::clipboard::state;

lazy_static::lazy_static! {
    static ref CLIPBOARD: Mutex<Clipboard> =Mutex::new(Clipboard::new().unwrap());
}

// Enum para representar el contenido del clipboard
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(ImageData<'static>, bool),
    ImageFile(PathBuf), // File path to image from Finder
    File(PathBuf),      // File path to any file from Finder
    Empty,
}

// Read from OS clipboard (Text or Image)
pub fn read_clipboard_content() -> Result<ClipboardContent, String> {
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;

    // First check for file list (Finder copy)
    if let Ok(paths_vec) = clipboard.get().file_list() {
        for path in &paths_vec {
            if is_image_file_path(path) {
                return Ok(ClipboardContent::ImageFile(path.clone()));
            }
        }
        for path in &paths_vec {
            if path.is_file() && !is_image_file_path(path) {
                println!("📁 Detected file from clipboard: {}", path.display());
                return Ok(ClipboardContent::File(path.clone()));
            }
        }
    }

    let mut cached_text: Option<String> = None;
    let mut screenshot_hint = false;

    if let Ok(text) = clipboard.get_text() {
        if !text.is_empty() {
            let trimmed = text.trim();
            let potential_path = PathBuf::from(trimmed);

            if trimmed.starts_with("file://") {
                if let Some(resolved) = path_from_file_url(trimmed) {
                    if resolved.is_file() && !is_image_file_path(&resolved) {
                        println!("📁 Detected file URL: {}", resolved.display());
                        return Ok(ClipboardContent::File(resolved));
                    }
                }
            }
            if is_image_file_path(&potential_path) {
                println!(
                    "📁 Detected image file from text: {}",
                    potential_path.display()
                );
                return Ok(ClipboardContent::ImageFile(potential_path));
            }
            if is_image_file_url(trimmed) {
                if let Some(resolved) = path_from_file_url(trimmed) {
                    if is_image_file_path(&resolved) {
                        println!("📁 Detected image file URL: {}", resolved.display());
                        return Ok(ClipboardContent::ImageFile(resolved));
                    }
                }
            }
        }

        screenshot_hint = looks_like_screenshot_text(&text);
        if !text.is_empty() {
            cached_text = Some(text);
        }
    }

    if let Ok(image) = clipboard.get_image() {
        //convert ImageData to 'static lifetime
        let owned_image = ImageData {
            width: image.width,
            height: image.height,
            bytes: image.bytes.into_owned().into(),
        };

        return Ok(ClipboardContent::Image(owned_image, screenshot_hint));
    }

    if let Some(text) = cached_text {
        return Ok(ClipboardContent::Text(text));
    }

    // If no cached text, fallback to plain get_text again
    if let Ok(text) = clipboard.get_text() {
        if !text.is_empty() {
            return Ok(ClipboardContent::Text(text));
        }
    }

    Ok(ClipboardContent::Empty)
}

// Helper to detect if text is an image file path
fn is_image_file_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if !path.exists() {
        return false;
    }
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
    {
        Some(ext) => matches!(
            ext.as_str(),
            "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "webp"
                | "bmp"
                | "tiff"
                | "svg"
                | "ico"
                | "heic"
                | "heif"
        ),
        None => false,
    }
}

/// Read text from OS clipboard (legacy function)
pub fn read_clipboard() -> Result<String, String> {
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;
    clipboard.get_text().map_err(|e| e.to_string())
}

/// Write text to OS clipboard
pub fn write_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())
}

// Write image to OS clipboard
pub fn write_clipboard_image(image_path: &str) -> Result<(), String> {
    use image::GenericImageView;

    // Load image from file
    let img = image::open(image_path).map_err(|e| format!("Failed to open image: {}", e))?;

    // COnvert to RGBA
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();

    //Create ImageData
    let image_data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: rgba.into_raw().into(),
    };

    // Write to clipboard
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;
    clipboard
        .set_image(image_data)
        .map_err(|e| e.to_string())
        .map(|_| state::request_skip_events(2))
}

fn is_image_file_url(text: &str) -> bool {
    text.trim_start().starts_with("file://")
}

fn path_from_file_url(url: &str) -> Option<PathBuf> {
    let decoded = percent_encoding::percent_decode_str(url.trim_start_matches("file://"))
        .decode_utf8()
        .ok()?;
    Some(PathBuf::from(decoded.as_ref()))
}

fn looks_like_screenshot_text(text: &str) -> bool {
    let lowered = text.trim().to_lowercase();
    if lowered.is_empty() {
        return false;
    }

    let screenshot_markers = [
        "screenshot",
        "screen shot",
        "captura de pantalla",
        "スクリーンショット",
        "截圖",
    ];

    if screenshot_markers
        .iter()
        .any(|marker| lowered.contains(marker))
    {
        return true;
    }

    lowered.ends_with(".png") || lowered.ends_with(".tiff") || lowered.ends_with(".heic")
}
