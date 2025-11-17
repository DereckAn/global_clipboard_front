use arboard::ImageData;
#[cfg(target_os = "macos")]
use image::image_dimensions;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
use std::process::Command;
use uuid::Uuid;

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
fn read_external_image_dimensions(path: &Path) -> Option<(u32, u32)> {
    if let Ok((width, height)) = image_dimensions(path) {
        return Some((width, height));
    }

    read_dimensions_with_sips(path)
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
    #[cfg(target_os = "macos")]
    {
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

    #[cfg(not(target_os = "macos"))]
    {
        // existing implementation unchanged (copy into images_dir)
        if !source_path.exists() {
            return Err(format!(
                "Source file does not exist: {}",
                source_path.display()
            ));
        }

        let original_name = source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");

        let original_extension = source_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|ext| ext.to_lowercase());

        let target_extension = match original_extension.as_deref() {
            Some("heic") | Some("heif") | Some("tiff") | Some("tif") => "png",
            Some(ext) => ext,
            None => "png",
        };

        let image_id = Uuid::new_v4();
        let file_name = format!("{}_{}.{}", original_name, image_id, target_extension);
        let thumb_name = format!("{}_{}_thumb.png", original_name, image_id);

        let full_path = images_dir.join(&file_name);
        let thumb_path = images_dir.join(&thumb_name);

        let mut dynamic_img: Option<DynamicImage> = None;

        match original_extension.as_deref() {
            Some("heic") | Some("heif") => {
                convert_heic_to_png(source_path, &full_path)?;
                dynamic_img = Some(
                    image::open(&full_path)
                        .map_err(|e| format!("Failed to load converted HEIC: {}", e))?,
                );
            }
            Some("tiff") | Some("tif") => {
                let img = image::open(source_path)
                    .map_err(|e| format!("Failed to load TIFF image: {}", e))?;
                img.save_with_format(&full_path, ImageFormat::Png)
                    .map_err(|e| format!("Failed to convert TIFF to PNG: {}", e))?;
                dynamic_img = Some(img);
            }
            _ => {
                fs::copy(source_path, &full_path)
                    .map_err(|e| format!("Failed to copy image file: {}", e))?;
            }
        }

        let img = match dynamic_img {
            Some(img) => img,
            None => image::open(&full_path)
                .map_err(|e| format!("Failed to load copied image: {}", e))?,
        };

        let (width, height) = img.dimensions();

        let thumbnail = img.resize(256, 256, imageops::FilterType::Lanczos3);
        thumbnail
            .save(&thumb_path)
            .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

        let file_size = fs::metadata(&full_path).map(|m| m.len()).unwrap_or(0);
        let file_hash = calculate_file_hash(&full_path)?;

        let mut info = StoredImageInfo {
            full_path,
            thumb_path,
            file_name,
            width,
            height,
            file_size,
            file_hash,
            original_extension,
            original_name: Some(original_name.to_string()),
            is_screenshot: looks_like_screenshot_name(original_name),
        };

        info.is_screenshot |= guess_screenshot_from_dimensions(info.width, info.height);

        Ok(info)
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

#[cfg(target_os = "macos")]
fn convert_heic_to_png(source: &Path, destination: &Path) -> Result<(), String> {
    let status = Command::new("sips")
        .arg("-s")
        .arg("format")
        .arg("png")
        .arg(source)
        .arg("--out")
        .arg(destination)
        .status()
        .map_err(|e| format!("Failed to run sips for HEIC conversion: {}", e))?;

    if !status.success() {
        return Err(format!(
            "sips failed to convert HEIC (exit code {:?})",
            status.code()
        ));
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn convert_heic_to_png(_source: &Path, _destination: &Path) -> Result<(), String> {
    Err("HEIC conversion is not supported on this platform.".to_string())
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
#[cfg(target_os = "macos")]
fn compute_external_hash(path: &Path) -> Result<String, String> {
    calculate_file_hash(path)
}
