pub fn test() {
    test1();
}

fn test1() {
    let foo = &Foo { x: 1, y: 2 };
    let Foo { x, y } = foo;
    // expected `i32`, found `&i32`
    // let i: i32 = x;
    // let i: i32 = y;
}

struct Foo {
    x: i32,
    y: i32,
}
