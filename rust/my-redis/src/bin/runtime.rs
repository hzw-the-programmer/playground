use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let op = say_world();
        println!("hello");
        op.await;
    });
}

async fn say_world() {
    println!("world");
}
