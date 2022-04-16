use std::env;

// scp src dst
// scp username@localhost:file ./
// scp username@localhost:/home/username/file ./

#[tokio::main]
async fn main() -> reqwest1::errors::Result<()> {
    let url = env::args().nth(1).expect("no url given");
    let dir = env::args().nth(2).expect("no dir given");
    reqwest1::download(&url, &dir).await?;
    Ok(())
}
