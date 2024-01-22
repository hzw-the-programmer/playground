use t1::Foo;

fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    print!("\ntest1\n\n");
    let mut dst = Foo { id: 1 };
    let src = Foo { id: 2 };
    std::mem::replace(&mut dst, src);
    println!("finish");
}

fn test2() {
    print!("\ntest2\n\n");
    let mut a = Foo { id: 1 };
    let mut b = Foo { id: 2 };
    std::mem::swap(&mut a, &mut b);
    println!("finish");
}

fn test3() {
    print!("\ntest3\n\n");
    let mut a = Foo { id: 1 };
    let mut b = Foo { id: 2 };
    println!("finish");
}
