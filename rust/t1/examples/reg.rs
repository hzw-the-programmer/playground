use regex::Regex;

fn main() {
    let reg = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    println!("mathes? {}", reg.is_match("2023-07-29"));
}
