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
        println!("create dir: {:?}", dir);
        fs::create_dir(dir); 
    }

    if !fname.is_file() {
        println!("create file: {:?}", fname);
        reqwest1::download(&url, &fname).await?;
    }

    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut r = m3u::EntryExtReader::new_ext(buf)?;
    for e in r.entry_exts() {
        let e = e?;
        if let m3u::Entry::Url(url) = e.entry {
            let url = Url::parse(url.as_str())?;
            let fname = reqwest1::fname(&url, &dir)?;
            tokio::spawn(async move {
                match reqwest1::download(&url, &fname).await {
                    Ok(()) => println!("ok"),
                    Err(e) => println!("{:?}", e)
                }
            });
        }
    }

    Ok(())
}
