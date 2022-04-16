use std::fs::{self, File};
use std::io::copy;
use std::path;

pub mod errors {
    use error_chain::error_chain;
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }
}

pub async fn download(url: &str, dir: &str) -> errors::Result<()> {
    let resp = reqwest::get(url).await?;

    let mut dest = {
        let fname = resp
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("file to download: '{}'", fname);
        let dir = path::PathBuf::from(dir);
        let fname = dir.join(fname);
        println!("will be downloaded under: {:?}", fname);
        if !dir.is_dir() {
            fs::create_dir(dir)?;
        }
        File::create(fname)?
    };

    let content = resp.text().await?;
    copy(&mut content.as_bytes(), &mut dest);

    Ok(())
}
