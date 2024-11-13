use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{Context, Poll, Waker};
use futures_util::FutureExt;

pub fn test() {
    // map();
    then();
}

fn map() {
    let mut cx = Context::from_waker(Waker::noop());

    let fut = async { Foo(0).await };
    let mut fut = fut.map(|x| x + 3);
    let mut fut = pin!(fut);

    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(4));
}

fn then() {
    println!("then: begin\n");

    let mut cx = Context::from_waker(Waker::noop());

    let fut = async {
        println!("async -> 2");
        2
    };
    let mut fut = fut.then(|i| {
        println!("closure");
        Foo(i)
    });
    let mut fut = pin!(fut);

    println!("then: begin poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("then: end poll: {:?}", r);

    println!("then: begin poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("then: end poll: {:?}", r);

    // thread 'main' panicked
    // println!("then: begin poll");
    // let r = fut.as_mut().poll(&mut cx);
    // println!("then: end poll: {:?}", r);

    println!("\nthen: end")
}

struct Foo(i32);

impl Future for Foo {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.0 % 2 == 0 {
            println!("Foo::poll -> Pending");
            self.0 += 1;
            Poll::Pending
        } else {
            println!("Foo::poll -> Ready({})", self.0);
            Poll::Ready(self.0)
        }
    }
}
