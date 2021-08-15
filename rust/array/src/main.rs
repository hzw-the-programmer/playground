fn main() {
    let x = [1, 2, 3];
    let y = x.map(|v| v + 1);
    println!("x: {:?}", x);
    println!("y: {:#?}", y);
}
