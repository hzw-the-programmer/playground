use futures::executor::block_on;
use futures::future::Future;

// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    println!("bar called");
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

async fn async_main() {
    let f = bar();
    println!("before await");
    let x = f.await;
    println!("{x}");
}

fn main() {
    block_on(async_main());
}
