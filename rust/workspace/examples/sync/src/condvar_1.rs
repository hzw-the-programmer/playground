use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn test() {
    test1();
}

fn test1() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut done = lock.lock().unwrap();
        *done = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    // expected `i32`, found `&Mutex<bool>`
    // let i: i32 = lock;
    // expected `i32`, found `&Condvar`
    // let i: i32 = cvar;
    let mut done = lock.lock().unwrap();
    while !*done {
        done = cvar.wait(done).unwrap();
    }
}
