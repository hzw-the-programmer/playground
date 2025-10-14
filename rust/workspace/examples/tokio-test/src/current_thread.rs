use tokio::runtime::{Builder, Handle};
// RUSTFLAGS="--cfg tokio_unstable" cargo run

pub fn test() {
    // test1();
    // test2();
    // test3();
    test4();
}

fn test1() {
    let rt = Builder::new_current_thread().build().unwrap();
    let metrics = rt.metrics();
    println!("{}", metrics.remote_schedule_count());
    rt.spawn(async {});
    println!("{}", metrics.remote_schedule_count());
}

fn test2() {
    // there is no reactor running, must be called from the context of a Tokio 1.x runtime
    let handle = Handle::current();
}

fn test3() {
    let rt = Builder::new_current_thread().build().unwrap();
    // there is no reactor running, must be called from the context of a Tokio 1.x runtime
    let handle = Handle::current();
}

fn test4() {
    let rt = Builder::new_current_thread().build().unwrap();
    let _guard = rt.enter();
    let handle = Handle::current();
    rt.spawn(async {});
    let metrics = rt.metrics();
    println!("{}", metrics.remote_schedule_count());
}
