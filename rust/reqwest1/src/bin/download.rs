use tempfile::Builder;
use std::io::copy;
use std::fs::{self,File};
use std::env;
use std::path;

// scp src dst
// scp username@localhost:file ./
// scp username@localhost:/home/username/file ./

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
    let url = env::args().nth(1).expect("no url given");
    let dir = env::args().nth(2).expect("no dir given");
    let dir = path::PathBuf::from(dir);
    let resp = reqwest::get(url).await?;
    let mut dest = {
        let fname = resp
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() {None} else {Some(name)})
            .unwrap_or("tmp.bin");
        println!("file to download: '{}'", fname);
        //let fname = tmp_dir.path().join(fname);
        let fname = dir.join(fname);
        println!("will be downloaded under: '{:?}'", fname);
        if dir.is_dir() {
            fs::remove_dir_all(&dir)?;
        }
        fs::create_dir(dir)?;
        File::create(fname)?
    };
    let content = resp.text().await?;
    copy(&mut content.as_bytes(), &mut dest);
    Ok(())
}
