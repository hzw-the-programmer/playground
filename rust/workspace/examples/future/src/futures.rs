use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{Context, Poll, Waker};
use futures_executor as executor;
use futures_util::{stream, FutureExt, Stream, StreamExt};
use std::thread;
use std::time::Duration;

pub fn test() {
    // map();
    // then();
    // left();
    // into_stream();
    // flatten();
    // flatten_stream_1();
    // flatten_stream_2();
    // fuse_1();
    // fuse_2();
    // inspect();
    // block_on_1();
    block_on_2();
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

fn flatten() {
    let fut = Foo(0);
    let fut = fut.map(|i| Foo(i + 1));
    let mut fut = pin!(fut.flatten());

    let mut cx = Context::from_waker(Waker::noop());

    println!("left: poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("left: poll end: {:?}\n", r);

    println!("left: poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("left: poll end: {:?}\n", r);

    println!("left: poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("left: poll end: {:?}", r);
}

fn flatten_stream_1() {
    let vec = vec![1, 2, 3];
    let fut = async {
        Foo(0).await;
        stream::iter(vec)
    };
    let st = fut.flatten_stream();
    let mut fut = pin!(st.collect::<Vec<i32>>());

    let mut cx = Context::from_waker(Waker::noop());

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

fn flatten_stream_2() {
    let vec = vec![1, 2, 3];
    let fut = async {
        Foo(0).await;
        stream::iter(vec)
    };
    let mut st = pin!(fut.flatten_stream());

    let mut cx = Context::from_waker(Waker::noop());

    println!("poll_next begin");
    let r = st.as_mut().poll_next(&mut cx);
    println!("poll_next end: {:?}\n", r);

    println!("poll_next begin");
    let r = st.as_mut().poll_next(&mut cx);
    println!("poll_next end: {:?}\n", r);

    println!("poll_next begin");
    let r = st.as_mut().poll_next(&mut cx);
    println!("poll_next end: {:?}\n", r);

    println!("poll_next begin");
    let r = st.as_mut().poll_next(&mut cx);
    println!("poll_next end: {:?}\n", r);

    println!("poll_next begin");
    let r = st.as_mut().poll_next(&mut cx);
    println!("poll_next end: {:?}", r);
}

fn fuse_1() {
    let mut fut = pin!(Foo(0));
    let mut cx = Context::from_waker(Waker::noop());
    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

fn fuse_2() {
    let mut fut = pin!(Foo(0).fuse());
    let mut cx = Context::from_waker(Waker::noop());

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

fn inspect() {
    let mut fut = pin!(Foo(0).inspect(|i| {
        // expected `i32`, found `&i32`
        // let ii: i32 = i;
        println!("inspect {i}")
    }));
    let mut cx = Context::from_waker(Waker::noop());

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    // println!("poll begin");
    // let r = fut.as_mut().poll(&mut cx);
    // println!("poll end: {:?}\n", r);
}

fn block_on_1() {
    executor::block_on(Foo(0));
}

fn block_on_2() {
    executor::block_on(Bar(0));
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

struct Bar(i32);

impl Future for Bar {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.0 % 2 == 0 {
            println!("Bar::poll -> Pending");
            self.0 += 1;
            let waker = cx.waker().clone();
            thread::spawn(move || {
                println!("thread: before sleep");
                thread::sleep(Duration::from_secs(5));
                println!("thread: after sleep");
                waker.wake();
            });
            Poll::Pending
        } else {
            println!("Bar::poll -> Ready({})", self.0);
            Poll::Ready(self.0)
        }
    }
}
