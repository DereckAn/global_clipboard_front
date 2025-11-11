use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub fn generate_thumbnail(path: &Path, target_path: &Path) -> Result<Option<PathBuf>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let Some(parent_dir) = target_path.parent() else {
        return Err("Invalid thumbnail target path".to_string());
    };

    let temp_dir = parent_dir.join(format!(".qlthumb-{}", Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create Quick Look temp dir: {e}"))?;

    let status = Command::new("qlmanage")
        .arg("-t")
        .arg("-s")
        .arg("512")
        .arg("-o")
        .arg(&temp_dir)
        .arg(path)
        .status();

    let result = match status {
        Ok(s) if s.success() => {
            let mut entries = fs::read_dir(&temp_dir)
                .map_err(|e| format!("Failed to read Quick Look output: {e}"))?
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .filter(|p| {
                    p.extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("png"))
                        .unwrap_or(false)
                })
                .collect::<Vec<PathBuf>>();

            entries.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());

            if let Some(png_path) = entries.pop() {
                match fs::rename(&png_path, target_path) {
                    Ok(_) => Ok(Some(target_path.to_path_buf())),
                    Err(err) => Err(format!("Failed to move Quick Look thumbnail: {err}")),
                }
            } else {
                Ok(None)
            }
        }
        Ok(_) | Err(_) => Ok(None),
    };

    let _ = fs::remove_dir_all(&temp_dir);
    result
}
