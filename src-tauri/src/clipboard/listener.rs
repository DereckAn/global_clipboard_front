use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use tokio::sync::mpsc::UnboundedSender;

use crate::clipboard::{detect_backend, ClipboardBackend};

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
fn spawn_native_listener(sender: UnboundedSender<ClipboardEvent>) {
    thread::spawn(move || {
        let handler = ClipboardEventHandler::new(sender);
        if let Err(err) = Master::new(handler).run() {
            eprintln!("Clipboard listener stopped: {err}");
        }
    });
}

/// Watch the Wayland clipboard via 'wl-paste --watch echo'.
/// Each clipboard change makes 'echo' print on linel; we turn each line
/// into one 'change' event. The line's content is irrelecvant - the
/// monitor re-reads the clipboard itself.
fn spawn_wayland_listener(sender: UnboundedSender<ClipboardEvent>) {
    thread::spawn(move || {
        let mut command = Command::new("wl-paste");
        command.arg("--watch").arg("echo").stdout(Stdio::piped());

        // Ask the kernel to send this child SIGTERM if the app (parent) dies,
        // so `wl-paste --watch` never lingers as an orphan after a kill/crash.
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::process::CommandExt;
            unsafe {
                command.pre_exec(|| {
                    libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
                    Ok(())
                });
            }
        }

        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(err) => {
                let _ = sender.send(ClipboardEvent::Error(format!(
                    "Failed to start `wl-paste --watch`:{err}"
                )));
                return;
            }
        };

        // Take ownership of the child's stdout so we can read it.
        let stdout = match child.stdout.take() {
            Some(stdout) => stdout,
            None => {
                let _ = sender.send(ClipboardEvent::Error(
                    "`wl-paste --watch` produced no stdout".to_string(),
                ));
                return;
            }
        };

        // Each line that arrives = one clipboard change.
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(_) => {
                    let _ = sender.send(ClipboardEvent::Changed);
                }
                Err(err) => {
                    let _ = sender.send(ClipboardEvent::Error(err.to_string()));
                    break;
                }
            }
        }
    });
}

/// Spawn the clipboard listener, choosing the backend at runtime.
pub fn spawn_clipboard_listener(sender: UnboundedSender<ClipboardEvent>) {
    match detect_backend() {
        ClipboardBackend::Wayland => spawn_wayland_listener(sender),
        ClipboardBackend::Native => spawn_native_listener(sender),
    }
}
