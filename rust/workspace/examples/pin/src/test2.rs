use core::ops::Deref;
use core::pin::Pin;

pub fn test() {}

struct Foo;

impl Foo {}

struct Bar(i32);

impl Bar {
    // type of `self` must be `Self` or a type that dereferences to it
    // fn pin_mut_2(self: Pin<Self>) {
    fn pin_mut(self: Pin<&mut Self>) {
        // expected `i32`, found `Pin<&mut Bar>`
        // let s = pin!(Bar);

        // expected `i32`, found `Pin<Bar>`
        // let s = Pin::new(Bar);

        // expected `i32`, found `Baz`
        // let s = Pin::new(Bar);
        // let s = *s;

        // expected `i32`, found `Bar`
        // let s = *self;

        // expected `i32`, found `Baz`
        // let s = **self;

        // expected `i32`, found `&Bar`
        // let s = &*self;

        // expected `i32`, found `&mut Bar`
        // let s = &mut *self;

        // expected `i32`, found `&Pin<&mut Bar>`
        // let s = &self;

        // expected `i32`, found `&Bar`
        // let s: &Bar = &self;

        // expected `i32`, found `&Baz`
        // let s: &Baz = &self;

        // let i: i32 = s;

        // cannot borrow as mutable
        // let s = &mut *self;
        // s.0 = 1;
    }
}

impl Deref for Bar {
    type Target = Baz;
    fn deref(&self) -> &Self::Target {
        &Baz
    }
}

struct Baz;
