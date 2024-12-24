pub fn test() {
    test1();
}

fn test1() {
    let foo = Foo { f1: 1, f2: Bar };
    let _bar = foo.into_bar();
}

struct Foo {
    f1: i32,
    f2: Bar,
}

// cannot move out of type `test2::Foo`, which implements the `Drop` trait
// impl Drop for Foo {
//     fn drop(&mut self) {}
// }

impl Foo {
    fn into_bar(self) -> Bar {
        self.f2
    }
}

struct Bar;

impl Drop for Bar {
    fn drop(&mut self) {}
}
