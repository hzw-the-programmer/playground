pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

fn test1() {
    println!("\ntest1: enter");
    let v = Bar;
    v.by_ref();
    v.by_val();
    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");
    let r = &Bar;
    r.by_ref();
    r.by_val();
    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");
    let b = Box::new(Bar);
    b.by_ref();
    b.by_val();
    println!("test3: leave");
}

fn test4() {
    println!("\ntest4: enter");
    let r = &&Bar;
    r.by_ref();
    r.by_val();
    println!("test4: leave");
}

fn test5() {
    println!("\ntest5: enter");
    let r = &&&Bar;
    r.by_ref();
    r.by_val();
    println!("test5: leave");
}

trait Foo {
    fn by_ref(&self);
    fn by_val(self);
}

impl<T: Foo> Foo for &T {
    fn by_ref(&self) {
        // let i: i32 = self;
        println!("&T.by_ref");
        // (**self).by_ref();
    }

    fn by_val(self) {
        // let i: i32 = self;
        println!("&T.by_val");
    }
}

// impl<T: Foo> Foo for Box<T> {
//     fn by_ref(&self) {
//         // let i: i32 = self;
//         println!("Box<T>.by_ref");
//         // (**self).by_ref();

//         // thread 'main' has overflowed its stack
//         // self.by_ref();
//         // (*self).by_ref();
//     }

//     fn by_val(self) {
//         // let i: i32 = self;
//         println!("Box<T>.by_val");
//     }
// }

// #[derive(Copy, Clone)]
struct Bar;

impl Foo for Bar {
    fn by_ref(&self) {
        println!("Bar.by_ref");
    }

    fn by_val(self) {
        println!("Bar.by_val");
    }
}
