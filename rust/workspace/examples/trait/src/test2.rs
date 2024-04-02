trait Write {
    fn write(&mut self);
}

struct Foo;

impl Write for Foo {
    fn write(&mut self) {
        println!("Foo.write");
    }
}

impl<W: Write> Write for &mut W {
    fn write(&mut self) {
        println!("&mut W.write");
    }
}

pub fn test() {
    let mut foo = Foo;
    foo.write();
    let foo_ref = &mut Foo;
    foo_ref.write();
    let foo_ref = &mut &mut Foo;
    foo_ref.write();
}
