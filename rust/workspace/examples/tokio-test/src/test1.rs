use core::{sync::atomic, time};
use std::thread;
use tokio::runtime;

pub fn test() {
    // test1();
    // test2();
    test3();
}

fn test1() {
    let rt = runtime::Runtime::new().unwrap();
    let _enter = rt.enter();
    // there is no reactor running, must be called from the context of a Tokio 1.x runtime
    tokio::spawn(async {});
}

fn test2() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, atomic::Ordering::SeqCst);
            format!("my-pool-{}", id)
        })
        .on_thread_start(|| println!("{} start", thread::current().name().unwrap()))
        .on_thread_stop(|| println!("{} stop", thread::current().name().unwrap()))
        .thread_keep_alive(time::Duration::from_secs(1))
        .on_thread_park(|| println!("{} park", thread::current().name().unwrap()))
        .on_thread_unpark(|| println!("{} unpark", thread::current().name().unwrap()))
        .worker_threads(2)
        .build()
        .unwrap();
    thread::sleep(time::Duration::from_secs(5));
}

fn test3() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, atomic::Ordering::SeqCst);
            format!("my-pool-{}", id)
        })
        .on_thread_start(|| println!("{} start", thread::current().name().unwrap()))
        .on_thread_stop(|| println!("{} stop", thread::current().name().unwrap()))
        .thread_keep_alive(time::Duration::from_secs(3))
        .on_thread_park(|| println!("{} park", thread::current().name().unwrap()))
        .on_thread_unpark(|| println!("{} unpark", thread::current().name().unwrap()))
        .worker_threads(2)
        .build()
        .unwrap();

    thread::sleep(time::Duration::from_secs(1));

    rt.spawn_blocking(|| println!("{} spawn blocking", thread::current().name().unwrap()));
    println!("after spawn blocking");

    thread::sleep(time::Duration::from_secs(6));
}
