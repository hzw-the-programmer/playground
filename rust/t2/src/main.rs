fn main() {
    let a = [1u8, 2, 3];
    // let n: () = a;
    // let n: () = &a;
    println!("{}", std::mem::size_of::<&[u8; 3]>());
    let b = &a;
    // let n: () = b;
    let c: &[u8] = &a;
    // let n: () = c;
    println!("{}", std::mem::size_of::<&[u8]>());
}
