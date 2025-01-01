pub fn test() {
    // test1();
    test2();
}

fn test1() {
    struct Foo {
        x: i32,
        y: i32,
    }

    let foo = &Foo { x: 1, y: 1 };
    let a = &[1, 2, 3];

    match (foo, a) {
        (&Foo { ref x, y }, &[ref first, second, _]) => {
            println!("x={}, y={}, first={}, second={}", *x, y, *first, second);
        }
    }

    match (foo, a) {
        (Foo { x, y }, [first, second, _]) => {
            println!("x={}, y={}, first={}, second={}", *x, *y, *first, *second);
        }
    }
}

fn test2() {
    struct Foo(i32);
    struct Bar(i32);
    enum Baz {
        Foo(Foo),
        Bar(Bar),
    }
    let baz1 = Baz::Foo(Foo(1));
    let baz2 = Baz::Foo(Foo(2));

    // match (&baz1, &baz2) {
    //     (Baz::Foo(a), Baz::Foo(b)) => {
    //         // expected `i32`, found `&Foo`
    //         let i: i32 = a;
    //         // expected `i32`, found `&Foo`
    //         let i: i32 = b;
    //     }
    //     _ => {}
    // }

    // match (baz1, baz2) {
    //     (Baz::Foo(ref a), Baz::Foo(ref b)) => {
    //         // expected `i32`, found `&Foo`
    //         let i: i32 = a;
    //         // expected `i32`, found `&Foo`
    //         let i: i32 = b;
    //     }
    //     _ => {}
    // }

    // match (&baz1, &baz2) {
    //     (Baz::Foo(ref a), Baz::Foo(ref b)) => {
    //         // expected `i32`, found `&Foo`
    //         let i: i32 = a;
    //         // expected `i32`, found `&Foo`
    //         let i: i32 = b;
    //     }
    //     _ => {}
    // }

    // match (baz1, baz2) {
    //     (Baz::Foo(a), Baz::Foo(b)) => {
    //         // expected `i32`, found `Foo`
    //         let i: i32 = a;
    //         // expected `i32`, found `Foo`
    //         let i: i32 = b;
    //     }
    //     _ => {}
    // }

    impl PartialEq for Foo {
        fn eq(&self, other: &Self) -> bool {
            println!("Foo::eq");
            self.0 == other.0
        }
    }

    // match (&baz1, &baz2) {
    //     (Baz::Foo(a), Baz::Foo(b)) => {
    //         let i = a == b;
    //     }
    //     _ => {}
    // }

    match (&baz1, &baz2) {
        (Baz::Foo(ref a), Baz::Foo(ref b)) => {
            let i = a == b;
        }
        _ => {}
    }
}
