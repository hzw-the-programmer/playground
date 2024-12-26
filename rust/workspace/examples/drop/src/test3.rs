pub fn test() {
    test1();
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
