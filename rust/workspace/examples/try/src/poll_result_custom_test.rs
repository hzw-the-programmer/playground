use crate::foo_bar::{Bar, Foo};
use crate::poll_result_custom::{PollCustom, ResultCustom};

pub fn test() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}

fn test1() {
    println!("\ntest1: enter");

    fn f() -> PollCustom<ResultCustom<Foo, Bar>> {
        println!("f: enter");

        let r = PollCustom::Pending;
        let r = r?;
        println!("f: {:?}", r);

        r.map(ResultCustom::Ok)
        // try {
        //     r
        // }
    }

    let r = f();
    println!("test1: {:?}", r);

    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");

    fn f() -> PollCustom<ResultCustom<Foo, Bar>> {
        println!("f: enter");

        let r = PollCustom::Ready(ResultCustom::Ok(Foo));
        let r = r?;
        println!("f: {:?}", r);

        // r.map(ResultCustom::Ok)
        try { r }
    }

    let r = f();
    println!("test2: {:?}", r);

    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");

    fn f() -> PollCustom<ResultCustom<Foo, Bar>> {
        println!("f: enter");

        let r = PollCustom::Ready(ResultCustom::Err(Bar));
        let r = r?;
        println!("f: {:?}", r);

        // r.map(ResultCustom::Ok)
        try { r }
    }

    let r = f();
    println!("test3: {:?}", r);

    println!("test3: leave");
}

fn test4() {
    println!("\ntest4: enter");

    fn f() -> PollCustom<ResultCustom<Foo, Bar>> {
        println!("f: enter");

        let r = ResultCustom::Err(Bar);
        let r = r?;
        println!("f: {:?}", r);

        // r.map(ResultCustom::Ok)
        try { r }
    }

    let r = f();
    println!("test4: {:?}", r);

    println!("test4: leave");
}

fn test5() {
    println!("\ntest5: enter");

    fn f() -> PollCustom<ResultCustom<Foo, Bar>> {
        println!("f: enter");

        let r = ResultCustom::Ok(Foo);
        let r = r?;
        println!("f: {:?}", r);

        // r.map(ResultCustom::Ok)
        try { PollCustom::Ready(r) }
    }

    let r = f();
    println!("test5: {:?}", r);

    println!("test5: leave");
}

fn test6() {
    println!("\ntest6: enter");

    fn f() -> PollCustom<ResultCustom<Foo, Bar>> {
        println!("f: enter");

        use core::ops::{ControlFlow, FromResidual, Try};

        let r = PollCustom::Ready(ResultCustom::Ok(Foo));
        // let r = PollCustom::Ready(ResultCustom::Err(Bar));

        // let r = r?;
        let r = match r.branch() {
            ControlFlow::Continue(c) => c,
            ControlFlow::Break(b) => return FromResidual::from_residual(b),
        };
        println!("f: {:?}", r);

        // try { r }
        Try::from_output(r)
    }

    let r = f();
    println!("test6: {:?}", r);

    println!("test6: leave");
}
