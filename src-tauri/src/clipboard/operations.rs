use arboard::{Clipboard, ImageData};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CLIPBOARD: Mutex<Clipboard> =Mutex::new(Clipboard::new().unwrap());
}

// Enum para representar el contenido del clipboard
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(ImageData<'static>),
    ImageFile(PathBuf), // File path to image from Finder
    Empty,
}

// Read from OS clipboard (Text or Image)
pub fn read_clipboard_content() -> Result<ClipboardContent, String> {
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;

    // First check for file list (Finder copy)
    if let Ok(paths) = clipboard.get().file_list() {
        if let Some(image_path) = paths.into_iter().find(|path| is_image_file_path(path)) {
            return Ok(ClipboardContent::ImageFile(image_path));
        }
    }

    // First check if there's a file path (Finder copy)
    if let Ok(text) = clipboard.get_text() {
        if !text.is_empty() {
            let potential_path = PathBuf::from(text.trim());
            if is_image_file_path(&potential_path) {
                println!(
                    "📁 Detected image file from text: {}",
                    potential_path.display()
                );
                return Ok(ClipboardContent::ImageFile(potential_path));
            }
            if is_image_file_url(&text) {
                if let Some(resolved) = path_from_file_url(&text) {
                    if is_image_file_path(&resolved) {
                        println!("📁 Detected image file URL: {}", resolved.display());
                        return Ok(ClipboardContent::ImageFile(resolved));
                    }
                }
            }
        }
    }

    // Try to get image data (screenshot, browser copy)
    if let Ok(image) = clipboard.get_image() {
        //convert ImageData to 'static lifetime
        let owned_image = ImageData {
            width: image.width,
            height: image.height,
            bytes: image.bytes.into_owned().into(),
        };

        return Ok(ClipboardContent::Image(owned_image));
    }

    // If no image, check for regular text
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
    clipboard.set_image(image_data).map_err(|e| e.to_string())
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
