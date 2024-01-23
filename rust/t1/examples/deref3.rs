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

    // cannot move out of dereference of `Bar<Foo>`
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

fn test3() {
    print!("\ntest3\n\n");

    let f = Foo { id: 1 };
    f.work();
    f.work_2();
}

fn test4() {
    print!("\ntest4\n\n");

    let f = &Foo { id: 1 };
    f.work();
    // cannot move out of `*f` which is behind a shared reference
    // f.work_2();
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}
