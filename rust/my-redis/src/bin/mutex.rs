use std::sync::{Mutex, MutexGuard};
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    //let m = Mutex::new(0);
    //increment_and_do_stuff(&m).await;
    //let handle = tokio::spawn(increment_and_do_stuff(m));
    //let ci = CanIncrement { m: Mutex::new(0) };
    //let handle = tokio::spawn(increment_and_do_stuff_1(ci));
    let m = tokio::sync::Mutex::new(0);
    let handle = tokio::spawn(increment_and_do_stuff_2(m));
    handle.await.unwrap();
}

async fn increment_and_do_stuff(m: Mutex<i32>) {
    {
        let mut mg: MutexGuard<i32> = m.lock().unwrap();
        *mg += 1;
    }
    yield_now().await;
}

async fn increment_and_do_stuff_1(ci: CanIncrement) {
    ci.increment();
    yield_now().await;
}

async fn increment_and_do_stuff_2(m: tokio::sync::Mutex<i32>) {
    let mut mg = m.lock().await;
    *mg += 1;
    yield_now().await;
}

struct CanIncrement {
    m: Mutex<i32>,
}

impl CanIncrement {
    fn increment(&self) {
        let mut mg = self.m.lock().unwrap();
        *mg += 1;
    }
}
