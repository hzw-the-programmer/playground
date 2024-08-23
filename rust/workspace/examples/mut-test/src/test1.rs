pub fn test() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    println!("\nteset1: enter");
    let mut f = Foo { id: 1 };
    let f = Foo { id: 2 };
    println!("teset1: leave");
}

fn test2() {
    println!("\nteset2: enter");
    let mut f = Foo { id: 1 };
    f = Foo { id: 2 };
    println!("teset2: leave");
}

fn test3() {
    println!("\ntest3: enter");
    let f = &mut Foo { id: 1 };
    f.id = 2;
    (*f).id = 3;
    // f = &mut Foo { id: 3 };
    println!("test3: leave");
}

fn test4() {
    println!("\ntest4: enter");
    let mut f = &Foo { id: 1 };
    // f.id = 2;
    // (*f).id = 3;
    f = &Foo { id: 4 };
    // println!("{}", f.id);
    println!("test4: leave");
}

struct Foo {
    id: u8,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}
