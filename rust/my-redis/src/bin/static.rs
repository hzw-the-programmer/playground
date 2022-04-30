#[tokio::main]
async fn main() {
    let vec = vec![1, 2, 3];
    //tokio::spawn(async {
    let handle = tokio::spawn(async move {
        println!("vec = {:?}", vec);
    });
    handle.await.unwrap();
}
