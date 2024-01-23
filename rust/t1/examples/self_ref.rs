fn main() {
    test1();
    test2();
}

struct Foo {
    id: i32,
    ptr: *const i32,
}

impl Foo {
    fn consume(self) {
        println!("{:p}", &self.id);
        println!("{:p}", &self.ptr);
        println!("{:p}", self.ptr);
    }
}

fn test1() {
    print!("\ntest1\n\n");

    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("{:p}", &f.id);
    println!("{:p}", &f.ptr);
    println!("{:p}", f.ptr);

    f.consume();
}

fn test2() {
    print!("\ntest2\n\n");

    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("{:p}", &f.id);
    println!("{:p}", &f.ptr);
    println!("{:p}", f.ptr);

    let func_ref = || {
        println!("{:p}", &f.id);
        println!("{:p}", &f.ptr);
        println!("{:p}", f.ptr);
    };
    println!();
    func_ref();

    let func = move || {
        println!("{:p}", &f.id);
        println!("{:p}", &f.ptr);
        println!("{:p}", f.ptr);
    };
    println!();
    func();
}
