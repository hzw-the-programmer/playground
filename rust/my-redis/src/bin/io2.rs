use std::env;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("path not given");
    let mut f = File::create(&path).await?;
    println!("{:?}", path);
    let n = f.write(b"hello world!").await?;
    println!("written {}", n);
    Ok(())
}
