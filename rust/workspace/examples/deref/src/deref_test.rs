use core::ops::Deref;

fn test1() {
    let i = 0;
    // error[E0614]: type `{integer}` cannot be dereferenced
    // let r = *i;
}

struct Foo {
    num: i32,
}

fn test2() {
    let foo = Foo { num: 1 };
    // error[E0614]: type `Foo` cannot be dereferenced
    // let r = *foo;
}

struct Bar {
    num: i32,
}

impl Deref for Bar {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

fn test3() {
    let bar = Bar { num: 1 };
    let r = *bar;
    bar.count_ones();

    fn t(i: &i32) {}

    t(&bar);
}

pub fn test() {}
