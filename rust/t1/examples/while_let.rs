fn main() {
    test1();
    test2();
}

struct Bar;

impl Bar {
    fn foo(&self) -> Foo {
        Foo { id: 1 }
    }
}

struct Foo {
    id: i32,
}

impl Foo {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

fn test1() {
    print!("\ntest1\n\n");
    let b = Bar;
    let mut i = 0;
    while let Some(n) = b.foo().id() {
        println!("in while");
        if i > 1 {
            break;
        }
        i += 1;
        println!("leave while")
    }
}

fn test2() {
    print!("\ntest2\n\n");
    let b = Bar;
    let mut i = 0;
    loop {
        let n = b.foo().id().unwrap();
        if i > 1 {
            break;
        }
        i += 1;
        println!("end")
    }
}
