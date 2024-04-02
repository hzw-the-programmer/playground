trait Write {
    fn write_value(self);
    fn write_ref(&self);
    fn write_ref_mut(&mut self);
}

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop {}", self.id);
    }
}

impl Write for Foo {
    fn write_value(self) {
        println!("write_value: {}", self.id);
    }
    fn write_ref(&self) {
        println!("write_ref: {}", self.id);
    }
    fn write_ref_mut(&mut self) {
        println!("write_ref_mut: {}", self.id);
    }
}

pub fn test() {
    let foo = Foo { id: 1 };
    foo.write_value();
    let foo = Foo { id: 2 };
    foo.write_ref();
    let mut foo = Foo { id: 3 };
    foo.write_ref_mut();
    foo = Foo { id: 4 };
    println!("finish");
}
