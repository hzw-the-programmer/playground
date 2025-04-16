use std::cell::Cell;
use std::thread;
use std::thread_local;

pub fn test() {
    // test1();
    // test2();
    // test3();
    // test4();
    test5();
}

struct Foo {
    i: Cell<i32>,
}

impl Foo {
    fn new(i: i32) -> Self {
        println!("Foo::new i={i}");
        Self { i: Cell::new(i) }
    }
}

thread_local! {
    static FOO: Foo = Foo::new(1);
}

fn test1() {
    println!("test1");
    FOO.with(|foo| {
        println!("Foo.with i={}", foo.i.get());
        foo.i.set(2);
    });
    FOO.with(|foo| {
        println!("Foo.with i={}", foo.i.get());
        foo.i.set(3);
    });

    thread::spawn(|| {
        println!("thread");
        FOO.with(|foo| {
            println!("Foo.with i={}", foo.i.get());
            foo.i.set(2);
        });
        FOO.with(|foo| {
            println!("Foo.with i={}", foo.i.get());
            foo.i.set(3);
        });
    })
    .join()
    .unwrap();
}

fn test2() {
    let /*mut*/ c1 = || {
        static mut I: i32 = 0;
        unsafe {
            println!("{I}");
            I += 1;
        }
    };
    let c2 = || {
        static mut I: i32 = 0;
        unsafe {
            println!("{I}");
            I += 1;
        }
    };
    // expected closure `{closure@examples\thread\src\thread_local_1.rs:53:18: 53:20}`
    // found closure `{closure@examples\thread\src\thread_local_1.rs:60:14: 60:16}`
    // c1 = c2;
    c1();
    c1();
    c1();
    c2();
    c2();
    c2();
}

fn test3() {
    let c = || {
        || {
            static mut I: i32 = 0;
            unsafe {
                println!("{I}");
                I += 1;
            }
        }
    };
    let c1 = c();
    let c2 = c();
    c1();
    c1();
    c1();
    c2();
    c2();
    c2();

    let mut c3 = c();
    let c4 = c();
    c3 = c4;
    c3();
}

fn test4() {
    fn f(f: impl Fn()) {
        f();
        f();
        f();
    }
    f(|| {
        static mut I: i32 = 0;
        unsafe {
            println!("{I}");
            I += 1;
        }
    });
    f(|| {
        static mut I: i32 = 0;
        unsafe {
            println!("{I}");
            I += 1;
        }
    });
}

struct Bar {
    f: fn(),
}

impl Bar {
    const fn new(f: fn()) -> Self {
        Bar { f }
    }
    fn call(&self) {
        println!("{:p}", self);
        (self.f)();
    }
}

const BAR_1: Bar = Bar::new(|| {
    static mut I: i32 = 0;
    unsafe {
        println!("{I}");
        I += 1;
    }
});
const BAR_2: Bar = Bar::new(|| {
    static mut I: i32 = 0;
    unsafe {
        println!("{I}");
        I += 1;
    }
});

impl Foo {
    fn call(&self) {
        println!("{:p}", self);
    }
}

const FOO_1: Foo = Foo { i: Cell::new(1) };

const CONST_A: i32 = 10;
const CONST_B: i32 = 10;

struct Baz {
    i: i32,
}
impl Baz {
    fn call(&self) {
        println!("{:p}", self);
    }
}
const BAZ_1: Baz = Baz { i: 1 };
const BAZ_2: Baz = Baz { i: 1 };

fn test5() {
    println!("&BAR_1 == &BAR_1: {}", std::ptr::eq(&BAR_1, &BAR_1));
    BAR_1.call();
    BAR_1.call();

    println!();
    println!("&BAR_1 == &BAR_2: {}", std::ptr::eq(&BAR_1, &BAR_2));
    BAR_1.call();
    BAR_2.call();
    BAR_2.call();

    println!();
    println!("&CONST_A == &CONST_B: {}", std::ptr::eq(&CONST_A, &CONST_B));
    println!("&CONST_A: {:p}", &CONST_A);
    println!("&CONST_B: {:p}", &CONST_B);

    println!();
    println!("&BAZ_1 == &BAZ_1: {}", std::ptr::eq(&BAZ_1, &BAZ_1));
    println!("&BAZ_1 == &BAZ_2: {}", std::ptr::eq(&BAZ_1, &BAZ_2));
    BAZ_1.call();
    BAZ_1.call();
    BAZ_2.call();
    BAZ_1.i = 2;
    println!("{}", BAZ_1.i);

    println!();
    println!("let r = &BAZ_1");
    let r = &BAZ_1;
    r.call();

    println!();
    println!("let r = &mut BAZ_1");
    let r = &mut BAZ_1;
    r.call();

    println!();
    println!("&FOO_1 == &FOO_1: {}", std::ptr::eq(&FOO_1, &FOO_1));
    FOO_1.call();
    FOO_1.call();
    FOO_1.i.set(2);
    println!("{}", FOO_1.i.get());

    println!();
    println!("let r = &FOO_1");
    let r = &FOO_1;
    r.call();

    println!();
    println!("let r = &mut FOO_1");
    let r = &mut FOO_1;
    r.call();
}
