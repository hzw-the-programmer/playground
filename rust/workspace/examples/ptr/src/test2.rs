use core::ptr::{self, NonNull};

pub fn test() {
    test1();
}

fn test1() {
    struct Foo(i32);
    impl Foo {
        fn by_ref(&self) {
            println!("Foo::by_ref");
        }
    }

    let f = Foo(0);
    f.by_ref();
    // no method named `by_ref` found for raw pointer `*const test2::test1::Foo` in the current scope
    // (&f as *const Foo).by_ref();
    // *(&f as *const Foo).by_ref();
    unsafe { (*(&f as *const Foo)).by_ref() };

    struct Bar {
        i: i32,
        f: Foo,
    }
    let b = Bar { i: 1, f: Foo(1) };
    // no method named `by_ref` found for raw pointer `*const test2::test1::Foo` in the current scope
    // *ptr::addr_of!(b.f).by_ref();

    struct Baz {
        i: i32,
        f: *const Foo,
    }
    let b = Baz { i: 1, f: &Foo(1) };
    // no method named `by_ref` found for raw pointer `*const *const test2::test1::Foo` in the current scope
    // *ptr::addr_of!(b.f).by_ref();

    // expected `i32`, found `*const NonNull<Foo>`
    // let i: i32 = ptr::addr_of!(b.f).cast::<NonNull<Foo>>();

    // expected `i32`, found `NonNull<Foo>`
    // let i: i32 = unsafe { *ptr::addr_of!(b.f).cast::<NonNull<Foo>>() };

    // expected `i32`, found `&mut NonNull<Foo>`
    // let i: i32 = unsafe { &mut *ptr::addr_of!(b.f).cast::<NonNull<Foo>>() };
}
