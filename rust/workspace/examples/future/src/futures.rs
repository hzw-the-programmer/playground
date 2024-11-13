use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{Context, Poll, Waker};
use futures_util::{FutureExt, StreamExt};

pub fn test() {
    // map();
    // then();
    // left();
    into_stream();
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

fn left() {
    let fut1 = async {
        println!("async -> true");
        true
    };
    let fut2 = async {
        println!("async -> false");
        false
    };
    let x = 6;
    let mut fut = if x < 10 {
        // `if` and `else` have incompatible types
        // fut1
        fut1.left_future()
    } else {
        // fut2
        fut2.right_future()
    };

    let mut cx = Context::from_waker(Waker::noop());

    let mut fut = pin!(fut);

    println!("left: poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("left: poll end: {:?}", r);
}

fn into_stream() {
    let fut = Foo(0);
    let st = fut.into_stream();
    let mut fut = pin!(st.collect::<Vec<i32>>());
    let mut cx = Context::from_waker(Waker::noop());

    println!("left: poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("left: poll end: {:?}\n", r);

    println!("left: poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("left: poll end: {:?}", r);
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
