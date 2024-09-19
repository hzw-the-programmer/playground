pub fn test() {
    test1();
}

fn test1() {
    let bar = Bar;
    bar.by_ref();
    bar.by_val();
}

trait Foo {
    fn by_ref(&self);
    fn by_val(self);
}

struct Bar;

impl Foo for Bar {
    fn by_ref(mut self: &Self) {
        println!("Foo.by_ref(mut self: &Self)");
    }
    fn by_val(mut self) {
        println!("Foo.by_val(mut self)");
    }
}
