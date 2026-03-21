use std::sync::Arc;
use std::thread;

mod atomic_counter;
use atomic_counter::AtomicCounter;

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
