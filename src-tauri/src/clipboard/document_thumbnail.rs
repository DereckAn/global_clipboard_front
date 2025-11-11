use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// Generates a thumbnail for the given document and saves it to the target directory.
///
/// # Arguments
/// * `path` - Path to the source document
/// * `target_dir` - Directory where the thumbnail should be saved
///
/// # Returns
/// * `Ok(Some(PathBuf))` - Thumbnail was successfully generated, returns path to thumbnail
/// * `Ok(None)` - Platform can't generate thumbnail for this file type (graceful fallback)
/// * `Err(String)` - Error occurred during generation
pub fn generate_document_thumbnail(path: &Path, target_dir: &Path) -> Result<Option<PathBuf>, String> {
    // Validate input path exists
    if !path.exists() {
        return Err(format!("Source file does not exist: {}", path.display()));
    }

    // Ensure target directory exists
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create thumbnail directory: {}", e))?;

    let thumb_path = build_cached_path(path, target_dir)?;
    if thumb_path.exists() {
        return Ok(Some(thumb_path));
    }

    #[cfg(target_os = "macos")]
    {
        if macos::generate_thumbnail(path, &thumb_path)? {
            return Ok(Some(thumb_path));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if windows::generate_thumbnail(path, &thumb_path)? {
            return Ok(Some(thumb_path));
        }
    }

    #[cfg(target_os = "linux")]
    {
        if linux::generate_thumbnail(path, &thumb_path)? {
            return Ok(Some(thumb_path));
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        let _ = (path, target_dir);
    }

    Ok(None)
}

/// Deletes a thumbnail file if it exists
pub fn delete_thumbnail(thumb_path: &Path) -> Result<(), String> {
    if thumb_path.exists() {
        std::fs::remove_file(thumb_path)
            .map_err(|e| format!("Failed to delete thumbnail: {}", e))?;
    }
    Ok(())
}

fn build_cached_path(path: &Path, target_dir: &Path) -> Result<PathBuf, String> {
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());

    if let Ok(metadata) = fs::metadata(path) {
        hasher.update(metadata.len().to_le_bytes());
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                hasher.update(duration.as_secs().to_le_bytes());
                hasher.update(duration.subsec_nanos().to_le_bytes());
            }
        }
    }

    let hash = format!("{:x}", hasher.finalize());
    Ok(target_dir.join(format!("{hash}.png")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_thumbnail_generation_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let fake_path = temp_dir.path().join("nonexistent.pdf");
        let target_dir = temp_dir.path().join("thumbnails");

        let result = generate_document_thumbnail(&fake_path, &target_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_thumbnail_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        let target_dir = temp_dir.path().join("thumbnails");
        assert!(!target_dir.exists());

        let _ = generate_document_thumbnail(&test_file, &target_dir);
        assert!(target_dir.exists());
    }

    #[test]
    fn test_delete_thumbnail() {
        let temp_dir = TempDir::new().unwrap();
        let thumb_path = temp_dir.path().join("test_thumb.png");
        fs::write(&thumb_path, "fake image data").unwrap();

        assert!(thumb_path.exists());
        delete_thumbnail(&thumb_path).unwrap();
        assert!(!thumb_path.exists());
    }

    #[test]
    fn test_delete_nonexistent_thumbnail() {
        let temp_dir = TempDir::new().unwrap();
        let thumb_path = temp_dir.path().join("nonexistent_thumb.png");

        let result = delete_thumbnail(&thumb_path);
        assert!(result.is_ok()); // Should not error on nonexistent file
    }
}
