pub fn test() {
    let foo = Foo;
    foo.by_ref();
    // ByRef::by_ref(foo); // mismatched types
    ByRef::by_ref(&foo);

    let rfoo = &Foo;
    rfoo.by_ref();
    ByRef::by_ref(rfoo);

    let rrfoo = &&Foo;
    rrfoo.by_ref();
    ByRef::by_ref(rrfoo);

    let rrrfoo = &&&Foo;
    rrrfoo.by_ref();
    // ByRef::by_ref(rrrfoo); // the trait bound `&&by_ref_2::Foo: by_ref_2::ByRef` is not satisfied
    ByRef::by_ref(*rrrfoo);
}

trait ByRef {
    fn by_ref(&self);
}

struct Foo;

impl ByRef for Foo {
    fn by_ref(&self) {
        println!("Foo.by_ref");
    }
}

impl ByRef for &Foo {
    fn by_ref(&self) {
        println!("&Foo.by_ref");
    }
}
