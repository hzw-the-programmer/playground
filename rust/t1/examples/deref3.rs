use std::ops::Deref;

struct Foo {
    id: i32,
}

impl Foo {
    fn work(&self) {
        println!("Foo.work");
    }

    fn work_2(self) {
        println!("Foo.work_2");
    }
}

struct Bar<T>(T);

impl<T> Deref for Bar<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("Bar.deref");
        &self.0
    }
}

fn test1() {
    print!("\ntest1\n\n");

    let foo = Foo { id: 1 };
    let b = Bar(foo);
    b.work();

    // b.work_2();
    // let f = *b;
    let f = b.0;
}

fn test2() {
    print!("\ntest2\n\n");

    let foo = Foo { id: 1 };
    let b = Box::new(foo);
    b.work();

    b.work_2();
    // let f = *b;
}

fn main() {
    test1();
    test2();
}
