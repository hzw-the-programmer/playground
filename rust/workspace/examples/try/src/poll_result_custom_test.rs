use crate::foo_bar::{Bar, Foo};
use crate::poll_result_custom::{PollCustom, ResultCustom};

pub fn test() {
    test1();
    test2();
    test3();
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
