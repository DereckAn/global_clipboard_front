use crate::clipboard::document_thumbnail;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StoredFileInfo {
    pub full_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
    pub file_mime_type: String,
    pub file_hash: String,
    pub original_extension: Option<String>,
    pub original_name: String,
}
pub struct PreparedFile {
    pub hash: String,
    pub mime_type: String,
    pub size: u64,
    pub extension: Option<String>,
    pub file_name: String,
}

pub fn prepare_file_metadata(source_path: &Path) -> Result<PreparedFile, String> {
    if !source_path.exists() {
        return Err(format!(
            "Source file does not exist: {}",
            source_path.display()
        ));
    }

    let extension = source_path
        .extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.to_lowercase());

    let hash = calculate_file_hash(source_path)?;
    let size = fs::metadata(source_path).map(|m| m.len()).unwrap_or(0);
    let mime_type = detect_mime_type(extension.as_deref());
    let base_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file")
        .to_string();

    Ok(PreparedFile {
        hash,
        mime_type,
        size,
        extension,
        file_name: base_name,
    })
}

pub fn store_prepared_file(
    prepared: &PreparedFile,
    source_path: &Path,
    files_dir: &Path,
) -> Result<StoredFileInfo, String> {
    let uuid = Uuid::new_v4();
    let file_name = match prepared.extension.as_deref() {
        Some(ext) => format!("{}_{}.{}", prepared.file_name, uuid, ext),
        None => format!("{}_{}", prepared.file_name, uuid),
    };

    let destination = files_dir.join(&file_name);
    fs::copy(source_path, &destination).map_err(|e| format!("Failed to copy file: {}", e))?;

    let original_name = source_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| prepared.file_name.as_str())
        .to_string();

    Ok(StoredFileInfo {
        full_path: destination,
        file_name,
        file_size: prepared.size,
        file_mime_type: prepared.mime_type.clone(),
        file_hash: prepared.hash.clone(),
        original_extension: prepared.extension.clone(),
        original_name,
    })
}

pub fn delete_file_from_disk(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("Failed to delete file: {}", e))?;
    }
    Ok(())
}

pub fn delete_file_thumbnail(thumbnail_path: &str) -> Result<(), String> {
    document_thumbnail::delete_thumbnail(Path::new(thumbnail_path))
}

pub fn delete_file_assets(file_path: &str, metadata_json: &str) -> Result<(), String> {
    delete_file_from_disk(file_path)?;
    if let Some(thumb_path) = extract_thumbnail_path(metadata_json) {
        delete_file_thumbnail(&thumb_path)?;
    }
    Ok(())
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

fn detect_mime_type(extension: Option<&str>) -> String {
    match extension {
        // Text files
        Some("txt") => "text/plain".to_string(),
        Some("md") => "text/markdown".to_string(),
        Some("csv") => "text/csv".to_string(),
        Some("xml") => "application/xml".to_string(),
        Some("json") => "application/json".to_string(),
        Some("html") | Some("htm") => "text/html".to_string(),
        Some("css") => "text/css".to_string(),
        Some("js") => "application/javascript".to_string(),

        // Documents
        Some("pdf") => "application/pdf".to_string(),
        Some("doc") => "application/msword".to_string(),
        Some("docx") => {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()
        }
        Some("xls") => "application/vnd.ms-excel".to_string(),
        Some("xlsx") => {
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()
        }
        Some("ppt") => "application/vnd.ms-powerpoint".to_string(),
        Some("pptx") => {
            "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string()
        }
        Some("odt") => "application/vnd.oasis.opendocument.text".to_string(),
        Some("ods") => "application/vnd.oasis.opendocument.spreadsheet".to_string(),
        Some("odp") => "application/vnd.oasis.opendocument.presentation".to_string(),

        // Archives
        Some("zip") => "application/zip".to_string(),
        Some("rar") => "application/x-rar-compressed".to_string(),
        Some("7z") => "application/x-7z-compressed".to_string(),
        Some("tar") => "application/x-tar".to_string(),
        Some("gz") => "application/gzip".to_string(),

        // Code files
        Some("rs") => "text/x-rust".to_string(),
        Some("py") => "text/x-python".to_string(),
        Some("java") => "text/x-java".to_string(),
        Some("cpp") | Some("cc") | Some("cxx") => "text/x-c++".to_string(),
        Some("c") => "text/x-c".to_string(),
        Some("h") | Some("hpp") => "text/x-c-header".to_string(),
        Some("go") => "text/x-go".to_string(),
        Some("rb") => "text/x-ruby".to_string(),
        Some("php") => "application/x-httpd-php".to_string(),
        Some("swift") => "text/x-swift".to_string(),
        Some("kt") => "text/x-kotlin".to_string(),
        Some("ts") => "application/typescript".to_string(),
        Some("sh") => "application/x-sh".to_string(),
        Some("bat") => "application/x-bat".to_string(),

        // Audio
        Some("mp3") => "audio/mpeg".to_string(),
        Some("wav") => "audio/wav".to_string(),
        Some("ogg") => "audio/ogg".to_string(),
        Some("m4a") => "audio/mp4".to_string(),
        Some("flac") => "audio/flac".to_string(),

        // Video
        Some("mp4") => "video/mp4".to_string(),
        Some("avi") => "video/x-msvideo".to_string(),
        Some("mov") => "video/quicktime".to_string(),
        Some("wmv") => "video/x-ms-wmv".to_string(),
        Some("mkv") => "video/x-matroska".to_string(),
        Some("webm") => "video/webm".to_string(),

        // Fonts
        Some("ttf") => "font/ttf".to_string(),
        Some("otf") => "font/otf".to_string(),
        Some("woff") => "font/woff".to_string(),
        Some("woff2") => "font/woff2".to_string(),

        // Other common files
        Some("apk") => "application/vnd.android.package-archive".to_string(),
        Some("dmg") => "application/x-apple-diskimage".to_string(),
        Some("exe") => "application/x-msdownload".to_string(),
        Some("deb") => "application/x-debian-package".to_string(),
        Some("rpm") => "application/x-rpm".to_string(),

        // Fallback for any other extension (excluding images)
        Some(ext) => mime_guess::from_ext(ext)
            .first_or_octet_stream()
            .essence_str()
            .to_string(),
        None => "application/octet-stream".to_string(),
    }
}

fn extract_thumbnail_path(metadata_json: &str) -> Option<String> {
    if metadata_json.trim().is_empty() {
        return None;
    }

    serde_json::from_str::<Value>(metadata_json)
        .ok()
        .and_then(|value| {
            value
                .get("thumbnail_path")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        })
}

#[cfg(target_os = "macos")]
pub fn write_file_to_clipboard(file_path: &str) -> Result<(), String> {
    use objc2::rc::Retained;
    use objc2::runtime::ProtocolObject;
    use objc2_app_kit::{NSPasteboard, NSPasteboardWriting};
    use objc2_foundation::{NSArray, NSString, NSURL};

    let pasteboard: Option<Retained<NSPasteboard>> = unsafe {
        use objc2::{msg_send, ClassType};
        msg_send![NSPasteboard::class(), generalPasteboard]
    };
    let Some(pasteboard) = pasteboard else {
        return Err("Failed to access pasteboard".into());
    };

    // Remove unnecessary unsafe - clearContents is safe to call
    pasteboard.clearContents();

    let new_path = NSString::from_str(file_path);
    // Remove unnecessary unsafe - fileURLWithPath is safe to call
    let url = NSURL::fileURLWithPath(&new_path);

    // Cast NSURL to NSPasteboardWriting protocol object
    let protocol_obj: Retained<ProtocolObject<dyn NSPasteboardWriting>> =
        ProtocolObject::from_retained(url);
    let objects: Retained<NSArray<ProtocolObject<dyn NSPasteboardWriting>>> =
        NSArray::from_retained_slice(&[protocol_obj]);

    // Remove unnecessary unsafe - writeObjects is safe to call
    let success = pasteboard.writeObjects(&objects);
    if success {
        Ok(())
    } else {
        Err("Failed to write file to clipboard".into())
    }
}
