use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut foo = pin!(Foo::new(3));
    let r = foo.as_mut().poll(&mut cx);
    println!("{r:?}");
    let r = foo.as_mut().poll(&mut cx);
    println!("{r:?}");
    let r = foo.as_mut().poll(&mut cx);
    println!("{r:?}");
    let r = foo.as_mut().poll(&mut cx);
    println!("{r:?}");

    let r = foo.as_mut().poll(&mut cx);
    println!("{r:?}");
}

fn test2() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = pin!(async {
        println!("async: begin");
        let mut foo = pin!(Foo::new(3));
        let r = foo.await;
        println!("async: r={r}");
        // move occurs because `foo` has type `Pin<&mut future_2::Foo>`, which does not implement the `Copy` trait
        // foo.await;
        println!("async: end");
        r
    });

    println!("test2: poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("test2: poll: {r:?}");

    println!("test2: poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("test2: poll: {r:?}");

    println!("test2: poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("test2: poll: {r:?}");

    println!("test2: poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("test2: poll: {r:?}");
}

struct Foo {
    count: usize,
}

impl Foo {
    fn new(count: usize) -> Self {
        Foo { count }
    }
}

impl Future for Foo {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("Foo::poll: count={}", self.count);
        if self.count > 0 {
            self.count -= 1;
            Poll::Pending
        } else {
            Poll::Ready(23)
        }
    }
}
