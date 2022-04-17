use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};
use url::Url;

pub mod errors {
    use error_chain::error_chain;
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
            UrlParse(url::ParseError);
            EntryReaderConstruction(m3u::EntryExtReaderConstructionError);
            ReadEntry(m3u::ReadEntryExtError);
        }
    }
}

pub async fn download(url: &Url, fname: &Path) -> errors::Result<u64> {
    let resp = reqwest::get(url.as_str()).await?;
    let src = resp.text().await?;
    let mut dst = File::create(fname)?;
    match copy(&mut src.as_bytes(), &mut dst) {
        Ok(size) => Ok(size),
        Err(err) => {
            fs::remove_file(fname)?;
            Err(errors::Error::from_kind(errors::ErrorKind::Io(err)))
        }
    }
}

pub fn fname(url: &Url, dir: &Path) -> errors::Result<PathBuf> {
    let fname = url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() {None} else {Some(name)})
        .unwrap_or("unamed");
    let fname = dir.join(fname);

    Ok(fname)
}
