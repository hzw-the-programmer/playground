use core::ops::Deref;

pub fn test() {
    test1();
}

fn test1() {}

struct Foo(Bar);
struct Bar;

impl Foo {
    fn f1(&self) {
        // expected `i32`, found `&Foo`
        // let i: i32 = self;

        // expected `i32`, found `Foo`
        // let i: i32 = *self;

        // expected `i32`, found `Bar`
        // let i: i32 = **self;

        // expected `i32`, found `&Bar`
        // let i: i32 = &**self;

        // expected `i32`, found `*const Bar`
        // let i: i32 = &**self as *const Bar;

        // expected `i32`, found `&*const Bar`
        // let i: i32 = &(&**self as *const Bar);
    }
}

impl Deref for Foo {
    type Target = Bar;
    fn deref(&self) -> &Self::Target {
        println!("Foo::Deref");
        &self.0
    }
}
