use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicCounter::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.inc();
                counter.dec();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.get());
}

struct AtomicCounter {
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
