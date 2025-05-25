pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let mut f = Foo { i: 1 };
    let f1 = f;
    // println!("{}", f.i);
    f = Foo { i: 2 };
    println!("{}", f.i);
}

fn test2() {
    let mut o = Some(Foo { i: 1 });
    if let Some(f) = o {
        o = Some(Foo { i: 2 });
        println!("{}", f.i);
    }
}

fn test3() {}

struct Foo {
    i: i32,
}
