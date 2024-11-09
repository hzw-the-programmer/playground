pub fn test() {
    test1();
}

fn test1() {
    println!("\ntest1: enter");

    let mut foo = Some(Foo {
        o1: Some(1),
        o2: Some(2),
    });

    match &mut foo {
        Some(Foo { o1, o2 }) => {
            // let i: i32 = o1;
            o1.take().unwrap();
            println!("{:?}", foo.unwrap().o1);
        }
        None => unreachable!(),
    }

    println!("test1: leave");
}

struct Foo {
    o1: Option<i32>,
    o2: Option<i32>,
}
