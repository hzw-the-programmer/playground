fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}

struct Foo {
    id: i32,
    ptr: *const i32,
}

impl Foo {
    fn consume(self) {
        println!("{:016p}", &self.id);
        println!("{:016p}", self.ptr);
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

fn test1() {
    print!("\ntest1\n\n");

    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("{:016p}", &f.id);
    println!("{:016p}", f.ptr);

    f.consume();

    println!("finish");
}

fn test2() {
    print!("\ntest2\n\n");

    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("{:016p}", &f.id);
    println!("{:016p}", f.ptr);

    let func_ref = || {
        println!("{:016p}", &f.id);
        println!("{:016p}", f.ptr);
    };
    println!();
    func_ref();

    let func = move || {
        println!("{:016p}", &f.id);
        println!("{:016p}", f.ptr);
    };
    println!();
    func();

    println!("finish");
}

fn test3() {
    print!("\ntest3\n\n");

    let f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    println!("stack");
    println!("{:016p}", &f.id);
    println!("{:016p}", f.ptr);

    let mut p = Box::new(f);
    p.ptr = &p.id;
    println!("heap");
    println!("{:016p}", &p.id);
    println!("{:016p}", p.ptr);

    let func_ref = || {
        println!("{:016p}", &p.id);
        println!("{:016p}", p.ptr);
    };
    println!();
    func_ref();

    let func = move || {
        println!("{:016p}", &p.id);
        println!("{:016p}", p.ptr);
    };
    println!();
    func();

    println!("finish");
}

struct Bar {
    id: i32,
}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("Bar {} drop", self.id);
    }
}

fn test4() {
    print!("\ntest4\n\n");

    let b = Bar { id: 1 };

    println!("{:016p}", &b.id);

    let h = std::thread::spawn(move || {
        println!("{:016p}", &b.id);
    });
    h.join();
    println!("finish");
}

fn test5() {
    print!("\ntest5\n\n");

    let h;
    {
        let b = Bar { id: 1 };
        println!("{:016p}", &b.id);

        h = std::thread::spawn(move || {
            println!("{:016p}", &b.id);
            println!("{}", b.id);
        });
    }
    println!("before join");
    h.join();
    println!("finish");
}

fn test6() {
    print!("\ntest6\n\n");

    let h;
    {
        let v = vec![1, 2, 3];

        h = std::thread::spawn(move || {
            println!("{:?}", v);
        });
    }
    println!("before join");
    h.join();
    println!("finish");
}
