fn main() {
    test1();
    test2();
    test3();
    test4();
}

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

impl Foo {
    fn work(self) {
        println!("stack: {:016p}", &self.id);
        println!("Foo.work");
    }
}

fn test1() {
    print!("\ntest1\n\n");

    let f = Foo { id: 1 };

    {
        let b = Box::new(f);
        println!("1");
    }

    println!("finish");
}

fn test2() {
    print!("\ntest2\n\n");

    let f = Foo { id: 1 };
    println!("stack: {:016p}", &f.id);

    {
        let b = Box::new(f);
        println!("heap : {:016p}", &b.id);
        {
            let n = b;
            println!("heap : {:016p}", &n.id);
            // let i: i32 = n;
            println!("1");
        }
        println!("2");
    }

    println!("finish");
}

fn test3() {
    print!("\ntest3\n\n");

    let f = Foo { id: 1 };
    println!("stack: {:016p}", &f.id);

    {
        let b = Box::new(f);
        println!("heap : {:016p}", &b.id);
        {
            let n = *b;
            println!("stack: {:016p}", &n.id);
            // let i: i32 = n;
            println!("1");
        }
        println!("2");
    }

    println!("finish");
}

fn test4() {
    print!("\ntest4\n\n");

    let f = Foo { id: 1 };
    println!("stack: {:016p}", &f.id);

    {
        let b = Box::new(f);
        println!("heap : {:016p}", &b.id);
        b.work();
        println!("1");
    }

    println!("finish");
}
