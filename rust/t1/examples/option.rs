use t1::Foo;

fn test_1() {
    let foo = Some(Foo { id: 1 });
    match foo {
        Some(v) => println!("Some: {}", v.id),
        None => println!("None"),
    }
    println!("finish");
}

fn test_2() {
    let foo = Some(Foo { id: 1 });
    match foo {
        Some(_) => println!("Some"),
        None => println!("None"),
    }
    println!("finish");
}

fn test_3() {
    let foo = Some(Foo { id: 1 });
    match foo {
        Some(ref v) => println!("Some: {}", v.id),
        None => println!("None"),
    }
    println!("finish");
}

fn main() {
    test_1();
    test_2();
    test_3();
}