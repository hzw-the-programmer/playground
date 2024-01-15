fn main() {
    test1();
    test2();
}

#[derive(Debug, Default)]
struct A {
    f1: i32,
    f2: u8,
}

fn test1() {
    println!("\ntest1\n");
    let a: A = Default::default();
    println!("{:?}", a);
    let b = A {
        f1: 1,
        ..Default::default()
    };
    println!("{:?}", b);
}

fn test2() {
    println!("\ntest2\n");
    let b: bool = Default::default();
    println!("{:?}", b);

    let b = bool::default();
    println!("{:?}", b);
}
