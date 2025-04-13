use futures_util::{Stream, StreamExt};
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut s = Foo::new();
    let mut f = pin!(s.next());

    println!("test1: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test1: poll result: {r:?}");

    println!("test1: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test1: poll result: {r:?}");

    println!("test1: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test1: poll result: {r:?}");

    println!("test1: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test1: poll result: {r:?}");
}

fn test2() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut s = Foo::new();
    let mut f = pin!(s.next());

    println!("test2: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test2: poll result: {r:?}");

    println!("test2: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test2: poll result: {r:?}");

    let mut f = pin!(s.next());

    println!("test2: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test2: poll result: {r:?}");

    println!("test2: poll");
    let r = f.as_mut().poll(&mut cx);
    println!("test2: poll result: {r:?}");
}

struct Foo(usize);

impl Foo {
    fn new() -> Self {
        Foo(0)
    }
}

impl Stream for Foo {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let i = self.0;
        println!("Foo:pool_next: i={i}");
        self.0 += 1;
        if i % 2 == 0 {
            Poll::Pending
        } else {
            Poll::Ready(Some(i))
        }
    }
}
