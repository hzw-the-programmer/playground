fn main() {
    test1();
}

trait Bar {
    fn f2(&self);
}

struct Foo;

impl Foo {
    fn f1(&self) {
        // let n: i32 = self;
        // let n: i32 = *self;
        // let n: i32 = &*self;
    }
}

// impl Bar for Foo {
//     fn f2(&self) {
//         println!("Foo.f2");
//         // (&*self).f2();
//     }
// }

impl Bar for &Foo {
    fn f2(&self) {
        // let n: i32 = self;
        println!("&Foo.f2");
    }
}

fn test1() {
    let f = Foo;
    f.f2();
    let a = &f;
    // let n: i32 = a;
    a.f2();
}
