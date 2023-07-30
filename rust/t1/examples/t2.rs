struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Fool drop {}", self.id);
    }
}

struct Bar {
    foo1: Foo,
    foo2: Foo,
}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("Bar drop");
    }
}

fn main() {
    let _bar = Bar {
        foo1: Foo { id: 1 },
        foo2: Foo { id: 2 },
    };
}
