use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let n = 10;
    let mut handles = Vec::with_capacity(n);
    let barrier = Arc::new(Barrier::new(n));
    for i in 0..n {
        let c = Arc::clone(&barrier);
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        handles.push(thread::spawn(move || {
            println!("before wait: {i}");
            let r = c.wait();
            println!("after wait: {i} is leader {}", r.is_leader());
        }));
    }
    // Wait for other threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}
