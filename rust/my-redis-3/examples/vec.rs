fn main() {
    let mut v = vec![0; 4096];
    println!("{}", v.capacity());
    println!("{}", v.len());

    v.reserve(1024);
    println!("{}", v.capacity());
    println!("{}", v.len());

    let b = &v[..];
    println!("{}", b.len());
}
