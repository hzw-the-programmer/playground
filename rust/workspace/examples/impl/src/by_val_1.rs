pub fn test() {
    let foo = Foo;
    foo.by_val();

    let rfoo = &Foo;
    // rfoo.by_val(); // cannot move out of `*rfoo` which is behind a shared reference
    // ByVal::by_val(rfoo); // the trait bound `&by_val_1::Foo: ByVal` is not satisfied
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
