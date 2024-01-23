trait Work {
    fn work(self);
}

struct Foo {
    id: i32,
}

impl Work for Foo {
    fn work(self) {
        println!("Foo.work");
    }
}

impl Work for &Foo {
    fn work(self) {
        println!("&Foo.work");
    }
}

fn main() {
    let f = Foo { id: 1 };
    f.work();
    let f = &Foo { id: 2 };
    f.work();
}
