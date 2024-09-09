pub fn test() {
    let foo = Foo;
    foo.by_val();

    let rfoo = &Foo;
    rfoo.by_val();
    ByVal::by_val(rfoo);
    // ByVal::by_val(*rfoo); // cannot move out of `*rfoo` which is behind a shared reference
}

trait ByVal {
    fn by_val(self);
}

struct Foo;

impl ByVal for Foo {
    fn by_val(self) {
        println!("Foo.by_val");
    }
}

impl ByVal for &Foo {
    fn by_val(self) {
        println!("&Foo.by_val");
    }
}
