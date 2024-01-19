use std::rc::Rc;

struct Foo {
    id: i32,
}

impl Foo {
    fn f1(&self) {
        println!("Foo.f1 called");
    }

    fn f2(&mut self) {
        println!("Foo.f2 called");
    }
}

fn main() {
    let mut rc = Rc::new(Foo { id: 1 });
    rc.f1();
    // rc.f2();
}
