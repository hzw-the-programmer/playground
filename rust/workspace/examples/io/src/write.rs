pub fn test() {
    test1();
}

fn test1() {
    use std::io::Write;

    let mut a = [0; 16];
    let mut s = &mut a[..];
    let _ = s.write(b"hello");
    println!("{:?}", s);
    // println!("{:?}", a);
    let _ = s.write(b" world!");
    println!("{:?}", s);
    println!("{:?}", a);
}
