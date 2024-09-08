pub fn test() {
    test1();
}

fn test1() {
    println!("\ntest1: begin");

    let f = Foo(1);
    f.by_ref();

    println!("test1: end");
}

struct Foo(i32);

#[derive(Copy, Clone)]
struct Bar(i32);

trait Baz {
    fn by_ref(&self);
    fn by_val(self);
}

impl Baz for Foo {
    fn by_ref(&self) {
        println!("Foo.by_ref: begin");
        self.by_val();
        // Baz::by_val(self); // the trait bound `&ref_val_2::Foo: Baz` is not satisfied
        // Baz::by_val(*self); // cannot move out of `*self` which is behind a shared reference
        println!("Foo.by_ref: end");
    }
    fn by_val(self) {
        println!("Foo.by_val");
    }
}

impl Baz for &Foo {
    fn by_ref(&self) {
        println!("&Foo.by_ref");
    }
    fn by_val(self) {
        println!("&Foo.by_val");
    }
}
