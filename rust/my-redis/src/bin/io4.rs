use std::env;
use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("path not given");
    let mut buf: &[u8] = b"hello hzw";
    let mut file = File::create(&path).await?;
    println!("{}", path);
    io::copy(&mut buf, &mut file).await?;
    Ok(())
}
