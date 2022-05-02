use std::env;

fn main() -> reqwest1::errors::Result<()> {
    let fname = env::args().nth(1).expect("fname not given");
    let tasks = reqwest1::parse_url(&fname)?;
    for task in tasks {
        println!("{:?}", task);
    }
    Ok(())
}
