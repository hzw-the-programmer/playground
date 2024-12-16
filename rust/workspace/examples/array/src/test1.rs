pub fn test() {
    test1();
}

fn test1() {
    let a = [Foo(0), Foo(1), Foo(2), Foo(3)];
    // this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
    // #[warn(array_into_iter)]
    // let mut i = a.into_iter();
    let mut i = IntoIterator::into_iter(a);
    i.next();
    println!("finish");
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("foo {} drop", self.0);
    }
}
