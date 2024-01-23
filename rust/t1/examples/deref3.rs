use std::ops::Deref;

struct Foo {
    id: i32,
}

impl Foo {
    fn work_ref(&self) {
        println!("Foo.work_ref");
        println!("{:016p}", self);
    }

    fn work_consume(self) {
        println!("Foo.work_consume");
        println!("{:016p}", &self);
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

    let f = Foo { id: 1 };
    println!("{:016p}", &f);
    f.work_ref();
    f.work_consume();
}

fn test2() {
    print!("\ntest2\n\n");

    let f = Foo { id: 1 };
    println!("{:016p}", &f);
    let b = Box::new(f);
    println!("{:016p}", b);
    b.work_ref();

    b.work_consume();
    // let f = *b;
}

fn test3() {
    print!("\ntest3\n\n");

    let f = &Foo { id: 1 };
    println!("{:016p}", f);
    f.work_ref();
    // cannot move out of `*f` which is behind a shared reference
    // f.work_consume();
}

fn test4() {
    print!("\ntest4\n\n");

    let foo = Foo { id: 1 };
    let b = Bar(foo);
    b.work_ref();

    // cannot move out of dereference of `Bar<Foo>`
    // b.work_consume();
    // let f = *b;
    let f = b.0;
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}
