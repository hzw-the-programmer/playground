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

fn test_5() {
    println!("\ntest_5\n");
    let o = Some(Foo { id: 1 });
    let n = o.map(|f| {
        // let n: i32 = f;
        println!("in map");
        f.id
    });
    println!("finish: {:?}", n);
}

fn test_6() {
    println!("\ntest_6\n");
    let o = Some(Foo { id: 1 });
    let o: Option<Foo> = None;
    let n = o.map_or(2, |f| {
        // let n: i32 = f;
        println!("in map_or");
        f.id
    });
    println!("finish: {:?}", n);
}

fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_6();
}
