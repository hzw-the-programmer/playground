use futures_executor::ThreadPool;
use std::thread;
use std::time::Duration;

pub fn test() {
    test1();
}

fn test1() {
    let pool = ThreadPool::builder()
        .after_start(|idx| {
            println!("{idx} after start");
        })
        .before_stop(|idx| {
            println!("{idx} before stop");
        })
        .create()
        .unwrap();

    thread::sleep(Duration::from_secs(3));
}
