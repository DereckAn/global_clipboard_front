pub mod monitor;
pub mod operations;
pub mod types;

pub use monitor::ClipboardMonitor;
pub use operations::{read_clipboard, write_clipboard};
