use std::fs::{self, File};
use std::io::{copy, BufRead, BufReader};
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
            Join(tokio::task::JoinError);
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

#[derive(Debug)]
pub struct Task {
    name: String,
    url: String
}

pub fn parse_url<P: AsRef<Path>>(path: P) -> errors::Result<Vec<Task>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut tasks = vec![];
    loop {
        let mut name = String::from("");
        let size = reader.read_line(&mut name)?;
        if size == 0 {
            break;
        }
        let mut url = String::from("");
        let size = reader.read_line(&mut url)?;
        if size == 0 {
            break;
        }
        tasks.push(Task{name, url});
        
    }
    Ok(tasks)
}

pub fn parse_m3u<P: AsRef<Path>>(path: P) -> errors::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut urls = vec![];
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with("#") {
            urls.push(line);
        }
    }
    Ok(urls)
}
