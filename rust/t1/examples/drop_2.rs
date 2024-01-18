struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

fn test1() {
    print!("test1 begin\n\n");
    let f = Foo { id: 1 };
    let f = Foo { id: 2 };
    println!("test1 end");
}

fn test2() {
    print!("\ntest2 begin\n\n");
    let mut f = Foo { id: 1 };
    f = Foo { id: 2 };
    println!("test2 end");
}

fn main() {
    test1();
    test2();
}
