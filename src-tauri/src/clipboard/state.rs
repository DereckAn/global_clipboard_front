use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

// AtomicI32 - a thread-safe integer. We us i32 (signed) so we can use -1 as "not set" value.
// AtomicUsize - a thread-safe unsigned integer. We use usize for counting events to skip.

// lazy_static! - creates global variables that are initialized once at first use.
lazy_static::lazy_static! {
    static ref SKIP_EVENTS: AtomicUsize = AtomicUsize::new(0);
    static ref PREVIOUS_APP_PID: AtomicI32 = AtomicI32::new(-1);
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
