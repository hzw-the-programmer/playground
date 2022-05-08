fn main() {
    let mut x = 42;
    let xb = &x;
    x = 2;
    println!("{}", xb);
}