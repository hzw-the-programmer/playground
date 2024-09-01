use core::ptr;

pub fn test() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("\ntest1: enter");

    let f1 = Foo(1);
    let f2 = Foo(2);

    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");

    let f1 = Foo(1);
    let f2 = unsafe { ptr::read(&f1) };

    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");

    let f1 = Foo(1);
    let mut f2 = Foo(2);
    unsafe { ptr::write(&mut f2, f1) };

    println!("test3: leave");
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}
