use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static::lazy_static! {
    static ref SKIP_EVENTS: AtomicUsize = AtomicUsize::new(0);
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
