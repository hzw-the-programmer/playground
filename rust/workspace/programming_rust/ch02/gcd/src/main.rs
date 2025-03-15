use gcd::gcd;
use std::env;
use std::str::FromStr;

fn main() {
    let mut nums = Vec::new();
    for arg in env::args().skip(1) {
        nums.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if nums.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = nums[0];
    for n in &nums[1..] {
        d = gcd(d, *n);
    }
    println!("The greatest common divisor of {:?} is {}", nums, d);
}
