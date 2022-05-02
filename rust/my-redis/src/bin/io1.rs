use std::env;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("path not given");
    let mut f = File::open(&path).await?;
    println!("{:?}", path);
    let mut buf = Vec::new();
    let n = f.read_to_end(&mut buf).await?;
    println!("{:?}", &buf[..n]);
    Ok(())
}
