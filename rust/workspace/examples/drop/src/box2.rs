pub fn test() {
    test1();
}

fn test1() {
    let b = Box::new(Bar {
        f1: unsafe { std::mem::uninitialized() },
        f2: Foo(2),
    });
    println!("leave");
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}

struct Bar {
    f1: Foo,
    f2: Foo,
}
