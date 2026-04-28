use core::ops::Deref;

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} dropped", self.id);
    }
}

struct Bar {
    id: i32,
}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("Bar {} dropped", self.id);
    }
}

struct Baz {
    // foo: Foo,
    bar: Bar,
}

impl Deref for Baz {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.bar
    }
}

pub fn test() {
    {
        let baz = Baz {
            // foo: Foo { id: 1 },
            bar: Bar { id: 1 },
        };
        // error[E0507]: cannot move out of dereference of `Baz`
        // move occurs because value has type `Bar`, which does not implement the `Copy` trait
        // let bar = *baz;
    }

    {
        let b = Box::new(Bar { id: 2 });
        // let bar = *b;
        {
            let bar = *b;
        }
        println!("xxx");
    }

    {
        let b = Box::pin(Bar { id: 3 });
        // error[E0507]: cannot move out of dereference of `Pin<Box<Bar>>`
        // move occurs because value has type `Bar`, which does not implement the `Copy` trait
        let bar = *b;
    }
}
