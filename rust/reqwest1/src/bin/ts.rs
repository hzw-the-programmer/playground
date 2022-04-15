use std::env;
use std::fs::File;
use std::io;

mod errors {
    use error_chain::error_chain;
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Construction(m3u::EntryExtReaderConstructionError);
            Read(m3u::ReadEntryExtError);
        }
    }
}

fn main() -> errors::Result<()> {
    let fname = env::args().nth(1).expect("fname not given");
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut r = m3u::EntryExtReader::new_ext(buf)?;
    for e in r.entry_exts() {
        let e = e?;
        if let m3u::Entry::Url(url) = e.entry {
            println!("{}", url.as_str());
        }
    }
    Ok(())
}
