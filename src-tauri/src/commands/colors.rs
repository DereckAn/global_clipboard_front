use crate::colors::{convert_color, ColorConversion};

#[tauri::command]
pub fn convert_color_formats(color: String) -> Result<ColorConversion, String> {
    convert_color(&color)
}

#[tauri::command]
pub fn extract_domain_from_url(url: String) -> Result<String, String> {
    // Simple domain extraction
    let url_trimmed = url.trim();

    // Remove protocol
    let without_protocol = url_trimmed
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    // Get domain (before first /)
    let domain = without_protocol.split('/').next().ok_or("Invalid URL")?;

    Ok(domain.to_string())
}
