fn main() {
    let tests = vec![
        test1, test2, test2_1, test3, test3_1, test4, test5, test6, test6_1, test7, test8,
    ];
    for (i, test) in tests.iter().enumerate() {
        print!("\n*** test {i} ***\n\n");
        test();
    }
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
    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("{:016p}", &f);
    println!("{:016p}", &f.id);
    println!("{:016p}", &f.ptr);
    println!("{:016p}", f.ptr);

    let func_ref = || {
        println!("{:016p}", &f);
        println!("{:016p}", &f.id);
        println!("{:016p}", &f.ptr);
        println!("{:016p}", f.ptr);
    };
    println!("\nref");
    func_ref();

    let func = move || {
        println!("{:016p}", &f);
        println!("{:016p}", &f.id);
        println!("{:016p}", &f.ptr);
        println!("{:016p}", f.ptr);
    };
    println!("\nmove");
    func();

    println!("finish");
}

fn test2_1() {
    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("{:016p}", &f);
    println!("{:016p}", &f.id);
    println!("{:016p}", &f.ptr);
    println!("{:016p}", f.ptr);

    {
        let func = || {
            println!("{:016p}", &f);
            println!("{:016p}", &f.id);
            println!("{:016p}", &f.ptr);
            println!("{:016p}", f.ptr);
        };

        println!("\nref");
        func();
    }

    {
        let func = move || {
            println!("{:016p}", &f.id);
            println!("{:016p}", &f.ptr);
            println!("{:016p}", f.ptr);
        };

        println!("\nmove");
        func();
    }

    {
        let func = move || {
            println!("{:016p}", &f);
            println!("{:016p}", &f.id);
            println!("{:016p}", &f.ptr);
            println!("{:016p}", f.ptr);
        };

        println!("\nmove");
        func();
    }

    println!("finish");
}

fn test3() {
    let f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    println!("stack");
    println!("{:016p}", &f);
    println!("{:016p}", &f.id);
    println!("{:016p}", &f.ptr);
    println!("{:016p}", f.ptr);

    let mut p = Box::new(f);
    p.ptr = &p.id;
    println!("heap");
    println!("{:016p}", p);
    println!("{:016p}", &p.id);
    println!("{:016p}", &p.ptr);
    println!("{:016p}", p.ptr);

    let func_ref = || {
        println!("{:016p}", p);
        println!("{:016p}", &p.id);
        println!("{:016p}", &p.ptr);
        println!("{:016p}", p.ptr);
    };
    println!("\nref");
    func_ref();

    let func = move || {
        println!("{:016p}", p);
        println!("{:016p}", &p.id);
        println!("{:016p}", &p.ptr);
        println!("{:016p}", p.ptr);
    };
    println!("\nmove");
    func();

    println!("finish");
}

fn test3_1() {
    let f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    println!("stack");
    println!("{:016p}", &f);
    println!("{:016p}", &f.id);
    println!("{:016p}", &f.ptr);
    println!("{:016p}", f.ptr);

    let mut p = Box::new(f);
    p.ptr = &p.id;
    println!("heap");
    println!("{:016p}", p);
    println!("{:016p}", &p.id);
    println!("{:016p}", &p.ptr);
    println!("{:016p}", p.ptr);

    {
        let func = || {
            println!("{:016p}", p);
            println!("{:016p}", &p.id);
            println!("{:016p}", &p.ptr);
            println!("{:016p}", p.ptr);
        };
        println!("\nref");
        func();
    }

    {
        let func = move || {
            println!("{:016p}", p);
            println!("{:016p}", &p.id);
            println!("{:016p}", &p.ptr);
            println!("{:016p}", p.ptr);
        };
        println!("\nmove");
        func();
    }

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
    let b = Bar { id: 1 };
    println!("{:016p}", &b.id);

    let h = std::thread::spawn(move || {
        println!("{:016p}", &b.id);
    });
    h.join();

    println!("finish");
}

fn test5() {
    let b = Bar { id: 1 };
    println!("{:016p}", &b);

    let h = std::thread::spawn(move || {
        println!("{:016p}", &b);
    });
    h.join();

    println!("finish");
}

fn test6() {
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

fn test6_1() {
    let h;
    {
        let b = Bar { id: 1 };
        println!("{:016p}", &b.id);

        h = std::thread::spawn(move || {
            println!("{:016p}", &b);
            println!("{}", b.id);
        });
    }
    println!("before join");
    h.join();
    println!("finish");
}

fn test7() {
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

fn test8() {
    let h;
    {
        let v = vec![1, 2, 3];

        h = std::thread::spawn(move || {
            println!("{:?}", v[0]);
        });
    }
    println!("before join");
    h.join();
    println!("finish");
}
