use std::sync::atomic::{AtomicI64, Ordering};

pub struct AtomicCounter {
    count: AtomicI64,
}

impl AtomicCounter {
    pub fn new(initial: i64) -> Self {
        Self {
            count: AtomicI64::new(initial),
        }
    }

    pub fn inc(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn dec(&self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn get(&self) -> i64 {
        self.count.load(Ordering::SeqCst)
    }
}
