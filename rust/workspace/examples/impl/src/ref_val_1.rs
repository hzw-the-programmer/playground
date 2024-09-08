pub fn test() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    println!("\ntest1: enter");

    let f = Foo(1);
    f.by_ref();
    // RefVal::by_ref(&f);
    f.by_val();
    // RefVal::by_val(f);
    // f.by_ref(); // borrow of moved value: `f`

    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: begin");

    let b = Bar(1);
    b.by_ref();
    // RefVal::by_ref(&b);
    b.by_val();
    // RefVal::by_val(b);
    b.by_ref();
    // RefVal::by_ref(&b);

    println!("test2: end");
}

fn test3() {
    println!("\ntest3: begin");

    let f = &Foo(1);
    f.by_ref();
    RefVal::by_ref(f);
    // f.by_val(); // cannot move out of `*f` which is behind a shared reference
    // RefVal::by_val(f); // the trait bound `&ref_val::Foo: RefVal` is not satisfied
    // RefVal::by_val(*f); // cannot move out of `*f` which is behind a shared reference

    println!("test3: end");
}

fn test4() {
    println!("\ntest4: begin");

    let b = &Bar(1);
    b.by_ref();
    // RefVal::by_ref(b);
    b.by_val();
    // RefVal::by_val(b); // the trait bound `&Bar: RefVal` is not satisfied
    // RefVal::by_val(*b);

    println!("test4: end");
}

struct Foo(i32);

#[derive(Copy, Clone)]
struct Bar(i32);

trait RefVal {
    fn by_ref(&self);
    fn by_val(self);
}

impl RefVal for Foo {
    fn by_ref(&self) {
        println!("Foo.by_ref");
    }
    fn by_val(self) {
        println!("Foo.by_val");
    }
}

impl RefVal for Bar {
    fn by_ref(&self) {
        println!("Bar.by_ref");
    }
    fn by_val(self) {
        println!("Bar.by_val");
    }
}
