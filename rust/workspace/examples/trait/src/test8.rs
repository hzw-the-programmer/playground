trait Write {
    fn write_val(self);
    fn write_ref(&self);
    fn write_ref_mut(&mut self);
}

// impl<T: Write> Write for &mut T {
//     fn write_val(self) {
//         println!("(&mut T).write_val()");
//     }
//     fn write_ref(&self) {
//         println!("(&mut T).write_ref()");
//     }
//     fn write_ref_mut(&mut self) {
//         println!("(&mut T).write_ref_mut()");
//     }
// }

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop {}", self.id);
    }
}

impl Write for Foo {
    fn write_val(self) {
        println!("Foo.write_val()");
    }
    fn write_ref(&self) {
        println!("Foo.write_ref()");
    }
    fn write_ref_mut(&mut self) {
        println!("Foo.write_ref_mut()");
    }
}

pub fn test() {
    let foo = &mut Foo { id: 1 };
    // foo.write_val();
    foo.write_ref();
    foo.write_ref_mut();

    let foo = &Foo { id: 2 };
    // foo.write_val();
    foo.write_ref();
    // foo.write_ref_mut();

    let foo = Foo { id: 3 };
    foo.write_val();

    let foo = Foo { id: 4 };
    foo.write_ref();

    let mut foo = Foo { id: 5 };
    foo.write_ref_mut();

    println!("finish");
}
