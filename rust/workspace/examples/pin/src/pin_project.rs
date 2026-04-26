use pin_project::pin_project;

struct Foo {
    f1: i32,
    f2: i32,
}

struct Bar {
    b1: i32,
    b2: i32,
}

#[pin_project]
struct Baz {
    foo: Foo,
    #[pin]
    bar: Bar,
}

pub fn test() {}
