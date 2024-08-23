use core::ops::Deref;

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
    test9();
    test11();
    test12();
}

fn test1() {
    println!("\ntest1: enter");

    let f = Foo;

    // f1(f);

    f1(&f);
    f1(&&f);
    f1(&&&f);
    f1(&&&&f);

    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");

    let f = Foo;

    f2(&f);
    f2(&&f);
    f2(&&&f);
    f2(&&&&f);

    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");

    let f = Foo;

    f3(&f);
    f3(&&f);
    f3(&&&f);
    f3(&&&&f);

    println!("test3: leave");
}

fn test4() {
    println!("\ntest4: enter");

    let f = Foo;

    f.bar_ref();
    f.bar_value();
    f.baz_ref();
    f.baz_value();

    println!("test4: leave");
}

fn test5() {
    println!("\ntest5: enter");

    let foo = &Foo;

    // let bar: Bar = foo;
    // let bar: Bar = *foo;
    let bar: Bar = **foo;

    println!("test5: leave");
}

fn test6() {
    println!("\ntest6: enter");

    let foo = &Foo;

    // let baz: Baz = foo;
    // let baz: Baz = *foo;
    // let baz: Baz = **foo;
    let baz: Baz = ***foo;

    println!("test6: leave");
}

fn test7() {
    println!("\ntest7: enter");

    let foo = &&Foo;

    // let baz: Baz = foo;
    // let baz: Baz = *foo;
    // let baz: Baz = **foo;
    // let baz: Baz = ***foo;
    let baz: Baz = ****foo;

    println!("test7: leave");
}

fn test8() {
    println!("\ntest8: enter");

    let foo = &Foo;

    let bar: &Bar = foo;

    println!("test8: leave");
}

fn test9() {
    println!("\ntest9: enter");

    let foo = &Foo;

    let bar: &Bar = &*foo;

    println!("test9: leave");
}

fn test10() {
    enum Foo {
        FooOne(Box<Bar>),
    }

    let foo = &Foo::FooOne(Box::new(Bar));
    match foo {
        Foo::FooOne(x) => {
            // let i: i32 = x;
            // let bar: Bar = x;
            let bar: &Bar = x;
            // let i: i32 = *x;
            // let bar = &*x;
            // let i: i32 = bar;
            let bar: &Bar = &*x;
        }
    }
}

fn test11() {}

fn test12() {}

fn f1(_f: &Foo) {
    println!("f1");
}

fn f2(_f: &Bar) {
    println!("f2");
}

fn f3(_f: &Baz) {
    println!("f3");
}

struct Foo;

impl Deref for Foo {
    type Target = Bar;
    fn deref(&self) -> &Self::Target {
        println!("Foo.deref");
        &Bar
    }
}

#[derive(Copy, Clone)]
struct Bar;

impl Bar {
    fn bar_ref(&self) {
        println!("Bar.bar_ref");
    }

    fn bar_value(self) {
        println!("Bar.bar_value");
    }
}

impl Deref for Bar {
    type Target = Baz;

    fn deref(&self) -> &Self::Target {
        println!("Bar.deref");
        &Baz
    }
}

#[derive(Copy, Clone)]
struct Baz;

impl Baz {
    fn baz_ref(&self) {
        println!("Baz.baz_ref");
    }

    fn baz_value(self) {
        println!("Baz.baz_value");
    }
}
