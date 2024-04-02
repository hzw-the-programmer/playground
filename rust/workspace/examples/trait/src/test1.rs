trait Write {
    fn write(&mut self);
}

struct Foo;

impl Write for Foo {
    fn write(&mut self) {
        println!("Foo.write");
    }
}

pub fn test() {
    let mut foo = Foo;
    foo.write();
    let foo_ref = &mut Foo;
    foo_ref.write();
}
