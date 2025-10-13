use tokio::runtime::Builder;
// RUSTFLAGS="--cfg tokio_unstable" cargo run

pub fn test() {
    test1();
}

fn test1() {
    let rt = Builder::new_current_thread().build().unwrap();
    let metrics = rt.metrics();
    println!("{}", metrics.remote_schedule_count());
    rt.spawn(async {});
    println!("{}", metrics.remote_schedule_count());
}
