use arboard::Clipboard;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CLIPBOARD: Mutex<Clipboard> =
Mutex::new(Clipboard::new().unwrap());
}

/// Read text from OS clipboard
pub fn read_clipboard() -> Result<String, String> {
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;
    clipboard.get_text().map_err(|e| e.to_string())
}

/// Write text to OS clipboard
pub fn write_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = CLIPBOARD.lock().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())
}
