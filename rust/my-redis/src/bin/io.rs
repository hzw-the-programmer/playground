use std::env;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("path not given");
    let mut f = File::open(&path).await?;
    println!("{:?}", path);
    let mut buf = [0; 10];
    let n = f.read(&mut buf).await?;
    println!("{:?}", &buf[..n]);
    Ok(())
}
