use cairo::{Context, ImageSurface};
use md5::{Digest, Md5};
use pango::FontDescription;
use pangocairo::{create_layout, show_layout};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub fn generate_thumbnail(path: &Path, target_path: &Path) -> Result<Option<PathBuf>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let Some(target_dir) = target_path.parent() else {
        return Err("Invalid thumbnail target path".to_string());
    };

    if let Some(existing) = lookup_freedesktop_cache(path) {
        if fs::copy(&existing, target_path).is_ok() {
            return Ok(Some(target_path.to_path_buf()));
        }
    }

    if is_pdf(path) {
        if generate_pdf_thumbnail(path, target_dir, target_path)? {
            return Ok(Some(target_path.to_path_buf()));
        }
    } else if is_text_like(path) {
        if generate_text_thumbnail(path, target_path)? {
            return Ok(Some(target_path.to_path_buf()));
        }
    }

    Ok(None)
}

fn lookup_freedesktop_cache(path: &Path) -> Option<PathBuf> {
    let home = env::var("HOME").ok()?;
    let cache_root = Path::new(&home).join(".cache/thumbnails/normal");
    let hash = md5_path(path);
    let candidate = cache_root.join(format!("{hash}.png"));
    candidate.exists().then_some(candidate)
}

fn md5_path(path: &Path) -> String {
    let mut hasher = Md5::new();
    hasher.update(path.to_string_lossy().as_bytes());
    format!("{:x}", hasher.finalize())
}

fn is_pdf(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

fn is_text_like(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_ascii_lowercase()),
        Some(ref ext)
            if matches!(
                ext.as_str(),
                "txt"
                    | "md"
                    | "markdown"
                    | "log"
                    | "json"
                    | "rs"
                    | "ts"
                    | "tsx"
                    | "js"
                    | "jsx"
                    | "css"
                    | "html"
                    | "java"
                    | "py"
                    | "go"
                    | "php"
            )
    )
}

fn generate_pdf_thumbnail(
    path: &Path,
    target_dir: &Path,
    target_path: &Path,
) -> Result<bool, String> {
    let temp_prefix = target_dir.join(format!(".pdftmp-{}", Uuid::new_v4()));
    let status = Command::new("pdftoppm")
        .arg("-png")
        .arg("-singlefile")
        .arg(path)
        .arg(&temp_prefix)
        .status();

    let png_path = temp_prefix.with_extension("png");
    let result = match status {
        Ok(s) if s.success() && png_path.exists() => {
            fs::rename(&png_path, target_path)
                .map_err(|e| format!("Failed to move PDF thumbnail: {e}"))?;
            Ok(true)
        }
        _ => Ok(false),
    };

    let _ = fs::remove_file(&png_path);
    let _ = fs::remove_file(&temp_prefix);
    result
}

fn generate_text_thumbnail(path: &Path, target_path: &Path) -> Result<bool, String> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let preview: String = content.lines().take(20).collect::<Vec<_>>().join("\n");

    let surface =
        ImageSurface::create(cairo::Format::ARgb32, 512, 512)
            .map_err(|e| format!("Failed to create surface: {e}"))?;
    let context = Context::new(&surface);

    context.set_source_rgb(0.11, 0.12, 0.15);
    context.rectangle(0.0, 0.0, 512.0, 512.0);
    context.fill().map_err(|e| format!("Failed to fill surface: {e}"))?;

    let layout = create_layout(&context).map_err(|e| format!("{e}"))?;
    let font_desc = FontDescription::from_string("JetBrains Mono 12");
    layout.set_font_description(Some(&font_desc));
    layout.set_width(480 * pango::SCALE);
    layout.set_text(&preview);

    context.set_source_rgb(0.85, 0.87, 0.92);
    context.move_to(16.0, 16.0);
    show_layout(&context, &layout);

    surface
        .write_to_png(target_path)
        .map_err(|e| format!("Failed to save text thumbnail: {e}"))?;

    Ok(true)
}
