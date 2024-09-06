pub fn test() {}

fn f1() -> Bar {
    Foo(1).into()
}

struct Foo(i32);

struct Bar(i32);

impl From<Foo> for Bar {
    fn from(f: Foo) -> Self {
        Bar(f.0)
    }
}
