use std::env;

/// Which Os clipboard backend we should talk to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardBackend {
    /// Native wayhland session  - use the 'wl-clipboard' tools (wl-paste / wl-copy)
    Wayland,
    /// Everything else: X11 on Linux, macOS, Windows — use arboard / clipboard-master.
    Native,
}

/// Decide the backend once at runtime, from the environment.
pub fn detect_backend() -> ClipboardBackend {
    // A Wayland compositor exports WAYLKAND_DISPLAY (e.g. " wayland-1")
    // IF it's present and non=empty we're on Wayland.

    match env::var("WAYLAND_DISPLAY") {
        Ok(val) if !val.is_empty() => ClipboardBackend::Wayland,
        _ => ClipboardBackend::Native,
    }
}
