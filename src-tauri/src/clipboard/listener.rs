use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::thread;
use tokio::sync::mpsc::UnboundedSender;

/// Events emitted by the clipboard listener.
#[derive(Debug)]
pub enum ClipboardEvent {
    Changed,
    Error(String),
}

struct ClipboardEventHandler {
    sender: UnboundedSender<ClipboardEvent>,
}

impl ClipboardEventHandler {
    fn new(sender: UnboundedSender<ClipboardEvent>) -> Self {
        Self { sender }
    }
}

impl ClipboardHandler for ClipboardEventHandler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // Ignore send errors – the receiver may have been dropped during shutdown.
        let _ = self.sender.send(ClipboardEvent::Changed);
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: std::io::Error) -> CallbackResult {
        let _ = self.sender.send(ClipboardEvent::Error(error.to_string()));
        CallbackResult::Next
    }
}

/// Spawn the platform clipboard listener in a dedicated thread.
pub fn spawn_clipboard_listener(sender: UnboundedSender<ClipboardEvent>) {
    thread::spawn(move || {
        let handler = ClipboardEventHandler::new(sender);
        if let Err(err) = Master::new(handler).run() {
            eprintln!("Clipboard listener stopped: {err}");
        }
    });
}
