pub fn test() {
    test1();
}

fn test1() {
    fn fn_1() {
        let foo = Foo(1);
        fn_2();
    }
    fn fn_2() {
        let foo = Foo(2);
        fn_3();
    }
    fn fn_3() {
        let foo = Foo(3);
        panic!("fn_3 paniced");
    }
    let foo = Foo(0);
    fn_1();
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}
