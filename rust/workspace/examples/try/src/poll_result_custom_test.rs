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
    let r = f1();
    println!("{:?}", r);
    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");
    let r = f2();
    println!("{:?}", r);
    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");
    let r = f3();
    println!("{:?}", r);
    println!("test3: leave");
}

fn test4() {
    println!("\ntest4: enter");
    let r = f4();
    println!("{:?}", r);
    println!("test4: leave");
}

fn test5() {
    println!("\ntest5: enter");
    let r = f5();
    println!("{:?}", r);
    println!("test5: leave");
}

fn test6() {
    println!("\ntest6: enter");
    let r = f6();
    println!("{:?}", r);
    println!("test6: leave");
}

fn f1() -> PollCustom<ResultCustom<Foo, Bar>> {
    let r = PollCustom::Pending;
    let r = r?;
    println!("after ?");
    r.map(ResultCustom::Ok)
    // try {
    //     r
    // }
}

fn f2() -> PollCustom<ResultCustom<Foo, Bar>> {
    let r = PollCustom::Ready(ResultCustom::Ok(Foo));
    let r = r?;
    println!("after ?");
    // r.map(ResultCustom::Ok)
    try { r }
}

fn f3() -> PollCustom<ResultCustom<Foo, Bar>> {
    let r = PollCustom::Ready(ResultCustom::Err(Bar));
    let r = r?;
    println!("after ?");
    // r.map(ResultCustom::Ok)
    try { r }
}

fn f4() -> PollCustom<ResultCustom<Foo, Bar>> {
    let r = ResultCustom::Err(Bar);
    let r = r?;
    println!("after ?");
    // r.map(ResultCustom::Ok)
    try { r }
}

fn f5() -> PollCustom<ResultCustom<Foo, Bar>> {
    let r = ResultCustom::Ok(Foo);
    let r = r?;
    println!("after ?");
    // r.map(ResultCustom::Ok)
    try { PollCustom::Ready(r) }
}

fn f6() -> PollCustom<ResultCustom<Foo, Bar>> {
    use core::ops::{ControlFlow, FromResidual, Try};

    let r = PollCustom::Ready(ResultCustom::Ok(Foo));
    // let r = PollCustom::Ready(ResultCustom::Err(Bar));

    // let r = r?;
    let r = match r.branch() {
        ControlFlow::Continue(c) => c,
        ControlFlow::Break(b) => return FromResidual::from_residual(b),
    };

    println!("after ?");

    // try { r }
    Try::from_output(r)
}
