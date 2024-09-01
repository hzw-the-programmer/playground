pub fn test() {
    test1();
    test2();
}

fn test1() {
    println!("\ntest1: enter");
    let v = Bar;
    v.by_ref();
    Foo::by_ref(&v);
    // v.by_val();
    Foo::by_val(v);
    println!("teset1: leave");
}

fn test2() {
    println!("\ntest2: enter");
    let r = &Bar;
    r.by_ref();
    Foo::by_ref(r);
    // r.by_val();
    Foo::by_val(&r);
    println!("teset2: leave");
}

trait Foo {
    fn by_ref(&self);
    fn by_val(self);
}

struct Bar;

impl Foo for Bar {
    fn by_ref(&self) {
        println!("Bar.by_ref()");
    }

    fn by_val(self) {
        println!("Bar.by_val");
    }
}

impl Foo for &Bar {
    fn by_ref(&self) {
        println!("&Bar.by_ref()");
    }

    fn by_val(self) {
        println!("&Bar.by_val");
    }
}
