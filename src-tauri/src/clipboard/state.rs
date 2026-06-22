use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

// AtomicI32 - a thread-safe integer. We us i32 (signed) so we can use -1 as "not set" value.
// AtomicUsize - a thread-safe unsigned integer. We use usize for counting events to skip.

// lazy_static! - creates global variables that are initialized once at first use.
lazy_static::lazy_static! {
    static ref SKIP_EVENTS: AtomicUsize = AtomicUsize::new(0);
    static ref PREVIOUS_APP_PID: AtomicI32 = AtomicI32::new(-1);
    // When the capture-folder watcher owns screenshots, the clipboard monitor
    // skips saving raw screenshot bytes so we don't get a duplicate entry.
    static ref SUPPRESS_SCREENSHOT_BYTES: AtomicBool = AtomicBool::new(false);
}

/// Enable/disable suppression of raw clipboard screenshot bytes.
/// Set to `true` when the folder watcher is active (it stores pointers instead).
pub fn set_suppress_screenshot_bytes(suppress: bool) {
    SUPPRESS_SCREENSHOT_BYTES.store(suppress, Ordering::SeqCst);
}

/// True when the clipboard monitor should skip saving raw screenshot bytes.
pub fn should_suppress_screenshot_bytes() -> bool {
    SUPPRESS_SCREENSHOT_BYTES.load(Ordering::SeqCst)
}

pub fn store_previous_app_pid(pid: i32) {
    PREVIOUS_APP_PID.store(pid, Ordering::SeqCst);
    // Syntax note: Ordering::SeqCst means "sequentially consistent" —
    // the strictest memory ordering, guarantees all threads see the
    // same value. Safe default for our case.
}

// Take_previous_app_pid - reads the PID and resets it to -1 (so it can only be used once)
// .swap() - Automatically replaces the value and returns the old one
pub fn take_previous_app_pid() -> i32 {
    PREVIOUS_APP_PID.swap(-1, Ordering::SeqCst)
}

pub fn request_skip_events(count: usize) {
    if count == 0 {
        return;
    }
    SKIP_EVENTS.fetch_add(count, Ordering::SeqCst);
}

pub fn take_skip_event() -> bool {
    loop {
        let current = SKIP_EVENTS.load(Ordering::SeqCst);
        if current == 0 {
            return false;
        }
        if SKIP_EVENTS
            .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_previous_app_pid() {
        store_previous_app_pid(12345);
        assert_eq!(take_previous_app_pid(), 12345);
    }

    #[test]
    fn test_take_previous_app_pid() {
        store_previous_app_pid(12345);
        assert_eq!(take_previous_app_pid(), 12345);
        assert_eq!(take_previous_app_pid(), -1);
    }

    #[test]
    fn test_request_skip_events() {
        // Reset global state left over from other tests
        while take_skip_event() {} // Clear any pending skip events

        request_skip_events(5);
        assert_eq!(SKIP_EVENTS.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn test_take_skip_event() {
        // Reset global state left over from other tests
        while take_skip_event() {} // Clear any pending skip events

        request_skip_events(1);
        assert_eq!(take_skip_event(), true);
        assert_eq!(take_skip_event(), false);
    }
}
