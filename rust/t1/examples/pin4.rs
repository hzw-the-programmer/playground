fn main() {
    test1();
}

struct Foo {
    id: i32,
    ptr: *const i32,
}

impl Foo {
    fn consume(self) {
        println!("{:p}", &self.id);
        println!("{:p}", self.ptr);
    }
}

fn test1() {
    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;
    println!("{:p}", &f.id);
    println!("{:p}", f.ptr);
    f.consume();
}
