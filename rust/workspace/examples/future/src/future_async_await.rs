use core::future::Future;
use core::task::{Context, Poll, Waker};
use std::pin::{pin, Pin};

pub fn test() {
    test1();
}

fn test1() {
    println!("\ntest1: begin");

    let mut cx = Context::from_waker(Waker::noop());

    // no method named `poll` found for opaque type `impl Future<Output = ()>` in the current scope
    // method `poll` found on `Pin<&mut impl Future<Output = ()>>`
    // f1().poll(&mut cx);

    let mut pinned = pin!(f1());
    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 1: {r:?}");
    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 2: {r:?}");
    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 3: {r:?}");

    // let mut future = Box::pin(f1());
    // future.as_mut().poll(&mut cx);

    println!("test1: end");
}

async fn f1() -> i32 {
    println!("f1: begin");

    let r = f2().await;

    println!("f1: end");

    r
}

async fn f2() -> i32 {
    println!("f2: begin");

    let r = Foo(1).await;

    println!("f2: end");

    r
}

struct Foo(i32);

impl Future for Foo {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        println!("Foo.poll");
        if self.0 < 3 {
            self.0 += 1;
            Poll::Pending
        } else {
            Poll::Ready(self.0)
        }
    }
}
