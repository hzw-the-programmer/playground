use std::env;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use url::Url;

#[tokio::main]
async fn main() -> reqwest1::errors::Result<()> {
    let url = Url::parse(&env::args().nth(1).expect("url not given"))?;
    let dir = env::args().nth(2).expect("dir not given");

    let dir = Path::new(&dir);
    let fname = reqwest1::fname(&url, &dir)?;

    if !dir.is_dir() {
        fs::create_dir(dir); 
    }

    if !fname.is_file() {
        match reqwest1::download(&url, &fname).await {
            Ok(size) => println!("download {:?} successful, size={}", fname.file_name().expect("file_name"), size),
            Err(err) => return Err(err)
        }
    }

    let mut handles = vec![];
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut r = m3u::EntryExtReader::new_ext(buf)?;
    for e in r.entry_exts() {
        let e = e?;
        if let m3u::Entry::Url(url) = e.entry {
            let url = Url::parse(url.as_str())?;
            let fname = reqwest1::fname(&url, &dir)?;
            if !fname.is_file() {
                let handle = tokio::spawn(async move {
                    match reqwest1::download(&url, &fname).await {
                        Ok(size) => println!("download {:?} successful, size={}", fname.file_name().expect("file_name"), size),
                        Err(err) => {
                            println!("download {:?} failed, err={:?}", fname.file_name().expect("file_name"), err);
                        }
                    }
                });
                handles.push(handle);
            }
            if handles.len() == 10 {
                for handle in handles {
                    handle.await;
                }
                handles = vec![];
            }
        }
    }

    Ok(())
}
