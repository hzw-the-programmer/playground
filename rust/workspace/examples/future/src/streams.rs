use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{Context, Poll, Waker};
use futures_util::{stream, Stream, StreamExt};

pub fn test() {
    // next_1();
    next_2();
}

fn next_1() {
    let mut st = stream::iter(1..=3);
    let mut fut = pin!(st.next());

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

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

fn next_2() {
    let mut st = Foo(0);
    let mut fut = pin!(st.next());

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

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

struct Foo(i32);

impl Stream for Foo {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let r = self.0;
        self.0 += 1;
        if r < 2 {
            println!("Foo::poll_next -> Pending");
            Poll::Pending
        } else if r < 4 {
            println!("Foo::poll_next -> Ready(Some({}))", r);
            Poll::Ready(Some(r))
        } else {
            println!("Foo::poll_next -> Ready(None)");
            Poll::Ready(None)
        }
    }
}
