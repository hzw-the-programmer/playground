use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("Body:\n{}", body);
    Ok(())
}