trait Write {
    fn write(self);
}

struct Foo;

impl Write for Foo {
    fn write(self) {
        println!("Foo.write");
    }
}

impl<W: Write> Write for &mut W {
    fn write(self) {
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
