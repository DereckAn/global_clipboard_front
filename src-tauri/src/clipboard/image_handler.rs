use arboard::ImageData;
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Save image from clipboard to disk WITH thumbnail
/// Returns (full_path, thumbnail_path, file_name, width, height, file_size, file_hash)
pub fn save_image_to_disk(
    image_data: &ImageData,
    images_dir: &Path,
) -> Result<(PathBuf, PathBuf, String, u32, u32, u64, String), String> {
    // Generate unique filename
    let image_id = Uuid::new_v4();
    let file_name = format!("{}.png", image_id);
    let thumb_name = format!("{}_thumb.png", image_id);

    let full_path = images_dir.join(&file_name);
    let thumb_path = images_dir.join(&thumb_name);

    // Convert ImageData to DynamicImage
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        image_data.bytes.to_vec(),
    )
    .ok_or("Failed to create image buffer")?;

    let dynamic_img = DynamicImage::ImageRgba8(img);

    // Save full image
    dynamic_img
        .save(&full_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    // Calculate SHA256 has of the saved file
    let file_hash = calculate_file_hash(&full_path)?;

    // Generate optimized thumbnail (256x256)
    // Use Lanczos3 for best quality downscaling
    let thumbnail = dynamic_img.resize(256, 256, imageops::FilterType::Lanczos3);

    thumbnail
        .save(&thumb_path)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    // Get file size
    let file_size = std::fs::metadata(&full_path).map(|m| m.len()).unwrap_or(0);

    println!(
        "✅ Saved image + thumbnail: {} ({}x{}) -> thumb: {} hash: {}",
        file_name,
        image_data.width,
        image_data.height,
        thumb_name,
        &file_hash[..12]
    );

    Ok((
        full_path,
        thumb_path,
        file_name,
        image_data.width as u32,
        image_data.height as u32,
        file_size,
        file_hash,
    ))
}
/// Copy image file directly from filesystem (Finder copies)
/// Returns (full_path, thumbnail_path, file_name, width, height, file_size, file_hash)
pub fn copy_image_file_to_storage(
      source_path: &Path,
      images_dir: &Path,
  ) -> Result<(PathBuf, PathBuf, String, u32, u32, u64, String), String> {
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

      let extension = source_path
          .extension()
          .and_then(|s| s.to_str())
          .unwrap_or("png");

      let image_id = Uuid::new_v4();
      let file_name = format!("{}_{}.{}", original_name, image_id, extension);
      let thumb_name = format!("{}_{}_thumb.png", original_name, image_id);

      let full_path = images_dir.join(&file_name);
      let thumb_path = images_dir.join(&thumb_name);

      fs::copy(source_path, &full_path)
          .map_err(|e| format!("Failed to copy image file: {}", e))?;

      let img = image::open(source_path).map_err(|e| format!("Failed to load image: {}", e))?;

    let (width, height) = img.dimensions();

    // Generate optimized thumbnail (256x256)
    let thumbnail = img.resize(256, 256, imageops::FilterType::Lanczos3);
    thumbnail
        .save(&thumb_path)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    // Get original file size
    let file_size = fs::metadata(&full_path).map(|m| m.len()).unwrap_or(0);

    // Calculate SHA256 hash of the original file for duplicate detection
    let file_hash = calculate_file_hash(&source_path)?;

    println!(
        "✅ Copied image file: {} ({}x{}) size: {} KB, hash: {} -> thumb: {}",
        file_name,
        width,
        height,
        file_size / 1024,
        &file_hash[..12], // Show first 12 chars of hash
        thumb_name
    );

    Ok((
        full_path, thumb_path, file_name, width, height, file_size, file_hash,
    ))
}

/// Delete image and thumbnail from disk
pub fn delete_image_from_disk(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    // Delete main image
    if path.exists() {
        std::fs::remove_file(path).map_err(|e| format!("Failed to delete image: {}", e))?;
    }

    // Delete thumbnail
    if let Some(parent) = path.parent() {
        if let Some(stem) = path.file_stem() {
            let thumb_path = parent.join(format!("{}_thumb.png", stem.to_string_lossy()));
            if thumb_path.exists() {
                std::fs::remove_file(thumb_path)
                    .map_err(|e| format!("Failed to delete thumbnail: {}", e))?;
            }
        }
    }

    Ok(())
}

/// Detect MIME type from file path or extension
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
        // Default to PNG for unknown types
        "image/png".to_string()
    }
}

/// Calculate SHA256 hash of a file for duplicate detection
pub fn calculate_file_hash(file_path: &Path) -> Result<String, String> {
    let mut file =
        fs::File::open(file_path).map_err(|e| format!("Failed to open file for hashing: {}", e))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8KB buffer for reading

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read file for hashing: {}", e))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
