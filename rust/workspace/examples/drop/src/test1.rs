#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

pub fn test() {
    test3();
}

fn test1() {
    struct Foo(u8);
    let mut f = Foo(1);
    let fdup = f;
    // f.0 = 2;
}

fn test2() {
    struct Foo {
        id: u8,
    }
    let mut f = Foo { id: 1 };
    let fdup = f;
    // f.id = 2;
}

fn test3() {
    println!("\ntest3: enter");

    struct Foo(u8);
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Foo {} drop", self.0);
        }
    }
    struct Bar(u8);
    impl Drop for Bar {
        fn drop(&mut self) {
            println!("Bar {} drop", self.0);
        }
    }
    struct Baz(Foo, Bar);
    impl Baz {
        fn into_foo(self) -> Foo {
            let r = self.0;
            println!("Baz.into_foo: leave");
            r
        }
        fn into_bar(self) -> Bar {
            self.1
        }
    }
    // rustc --explain E0509
    // impl Drop for Baz {
    //     fn drop(&mut self) {
    //         println!("Baz drop");
    //     }
    // }

    let baz = Baz(Foo(1), Bar(1));
    let foo = baz.into_foo();

    println!("test3: leave");
}
