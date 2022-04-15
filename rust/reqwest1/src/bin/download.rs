use tempfile::Builder;
use std::io::copy;
use std::fs::File;

mod errors {
    use error_chain::error_chain;
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }
}

#[tokio::main]
async fn main() -> errors::Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    //let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let url = std::env::args().nth(1).expect("no url given");
    let resp = reqwest::get(url).await?;
    let mut dest = {
        let fname = resp
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() {None} else {Some(name)})
            .unwrap_or("tmp.bin");
        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be downloaded under: '{:?}'", fname);
        File::create(fname)?
    };
    let content = resp.text().await?;
    copy(&mut content.as_bytes(), &mut dest);
    Ok(())
}
