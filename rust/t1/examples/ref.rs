trait Work {
    fn work(self);
}

trait WorkRef {
    fn work_ref(&self);
}

struct Foo {
    id: i32,
}

impl Work for Foo {
    fn work(self) {
        println!("Foo.work");
    }
}

impl WorkRef for Foo {
    fn work_ref(&self) {
        println!("Foo.work_ref");
    }
}

impl Work for &Foo {
    fn work(self) {
        println!("&Foo.work");
    }
}

impl WorkRef for &Foo {
    fn work_ref(&self) {
        println!("&Foo.work_ref");
    }
}

fn test1() {
    print!("\ntest1\n\n");
    let f = Foo { id: 1 };
    f.work();
    let f = &Foo { id: 2 };
    f.work();
}

fn test2() {
    print!("\ntest2\n\n");
    let f = Foo { id: 1 };
    f.work_ref();
    let f = &Foo { id: 2 };
    f.work_ref();
    let f = &&Foo { id: 2 };
    f.work_ref();
}

fn main() {
    test1();
    test2();
}
