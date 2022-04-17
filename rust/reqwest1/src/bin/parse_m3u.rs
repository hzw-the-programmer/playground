use std::env;

fn main() -> reqwest1::errors::Result<()> {
    let fname = env::args().nth(1).expect("fname not given");
    for url in reqwest1::parse_m3u(&fname)? {
        println!("{}", url);
    }
    Ok(())
}
