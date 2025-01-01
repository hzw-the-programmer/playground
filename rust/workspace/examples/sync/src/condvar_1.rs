use core::time::Duration;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn test() {
    // test1();
    test2();
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

fn test2() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair1 = pair.clone();
    let jh1 = thread::spawn(move || {
        let (lock, cvar) = &*pair1;
        let mut done = lock.lock().unwrap();
        while !*done {
            done = cvar.wait(done).unwrap();
        }
        println!("thread 1 done");
    });

    let pair2 = pair.clone();
    let jh2 = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut done = lock.lock().unwrap();
        while !*done {
            done = cvar.wait(done).unwrap();
        }
        println!("thread 2 done");
    });

    thread::sleep(Duration::from_secs(1));
    let (lock, cvar) = &*pair;
    {
        let mut done = lock.lock().unwrap();
        *done = true;
        cvar.notify_all();
        // cvar.notify_one();
        println!("notified");
        thread::sleep(Duration::from_secs(2));
    }

    let _ = jh1.join();
    let _ = jh2.join();
}
