trait Trait1 {
    fn f1(&self);
    fn f2(self);
}

struct Foo;

impl Trait1 for Foo {
    fn f1(&self) {
        println!("Trait1::f1 Foo");
    }

    fn f2(self) {
        println!("Trait1::f2 Foo");
    }
}

impl Trait1 for &Foo {
    fn f1(&self) {
        println!("Trait1::f1 &Foo");
    }

    fn f2(self) {
        println!("Trait1::f2 &Foo");
    }
}

// impl Trait1 for &&Foo {
//     fn f1(&self) {
//         println!("Trait1::f1 &&Foo");
//     }

//     fn f2(self) {
//         println!("Trait1::f2 &&Foo");
//     }
// }

pub fn test() {
    let rfoo = &Foo;
    rfoo.f1();
    rfoo.f2();

    let rrfoo = &&Foo;
    rrfoo.f1();
    rrfoo.f2();
}
