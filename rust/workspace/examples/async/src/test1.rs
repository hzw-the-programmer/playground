use core::future::Future;

pub fn test() {
    test1();
}

fn test1() {
    println!("\ntest1: enter");

    type_test(async_1);

    println!("test1: leave");
}

async fn async_1() -> i32 {
    1
}

fn type_test<F: FnOnce() -> Fut, Fut: Future<Output = i32>>(_f: F) {}
