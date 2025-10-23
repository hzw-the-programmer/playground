mod deepseek;

#[tokio::main]
async fn main() {
    deepseek::main().await;
}
