fn main() {
    test1();
    test2();
    test3();
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
