pub fn test() {
    // test1();
    // test2();
    // read();
    write();
}

fn test1() {
    struct Foo(Bar, Baz);
    struct Bar;
    struct Baz;
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Foo drop");
        }
    }
    impl Drop for Bar {
        fn drop(&mut self) {
            println!("Bar drop");
        }
    }
    impl Drop for Baz {
        fn drop(&mut self) {
            println!("Baz drop");
        }
    }

    let _foo = Foo(Bar, Baz);
    println!("finish");
}

fn test2() {
    struct Foo(Bar, Baz);
    struct Bar;
    struct Baz;
    impl Foo {
        fn into_bar(self) -> Bar {
            println!("Foo::into_bar");
            self.0
        }
    }
    // cannot move out of type `test3::test2::Foo`, which implements the `Drop` trait
    // impl Drop for Foo {
    //     fn drop(&mut self) {
    //         println!("Foo drop");
    //     }
    // }
    impl Drop for Bar {
        fn drop(&mut self) {
            println!("Bar::drop");
        }
    }
    impl Drop for Baz {
        fn drop(&mut self) {
            println!("Baz::drop");
        }
    }

    let foo = Foo(Bar, Baz);
    let _bar = foo.into_bar();
    println!("finish");
}

fn read() {
    use core::ptr;
    struct Foo(i32);
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Foo {} drop", self.0);
        }
    }

    let f1 = Foo(1);
    unsafe {
        let _f1 = ptr::read(&f1);
    }
    println!("finish");
}

fn write() {
    use core::ptr;
    struct Foo(i32);
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Foo {} drop", self.0);
        }
    }

    let mut f1 = Foo(1);
    unsafe {
        let f2 = Foo(2);
        ptr::write(&mut f1, f2);
    }
    println!("finish");
}
