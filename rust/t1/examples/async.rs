// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    println!("1");
    let future = hello_world(); // Nothing is printed
    // let n: i32 = future;
    println!("2");
    block_on(future); // `future` is run and "hello, world!" is printed
    println!("3");
}
