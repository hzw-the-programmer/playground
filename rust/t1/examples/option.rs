use t1::Foo;

fn test_1() {
    println!("\ntest_1\n");
    let foo = Some(Foo { id: 1 });
    match foo {
        Some(v) => println!("Some: {}", v.id),
        None => println!("None"),
    }
    println!("finish");
}

fn test_2() {
    println!("\ntest_2\n");
    let foo = Some(Foo { id: 1 });
    match foo {
        Some(_) => println!("Some"),
        None => println!("None"),
    }
    println!("finish");
}

fn test_3() {
    println!("\ntest_3\n");
    let foo = Some(Foo { id: 1 });
    match foo {
        Some(ref v) => println!("Some: {}", v.id),
        None => println!("None"),
    }
    println!("finish");
}

fn test_4() {
    println!("\ntest_4\n");
    let mut foo = Some(Foo { id: 1 });
    match foo {
        Some(ref v) => println!("Some: {}", v.id),
        None => println!("None"),
    }
    foo = None;
    println!("finish");
}

fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
}
