use core::future::Future;
use core::task::{ready, Context, Poll, Waker};
use std::pin::{pin, Pin};

pub fn test() {
    test1();
}

fn test1() {
    println!("\ntest1: begin");

    let mut cx = Context::from_waker(Waker::noop());

    let mut pinned = pin!(Future1::StateBegin);

    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 1: {r:?}");

    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 2: {r:?}");

    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 3: {r:?}");

    println!("test1: end");
}

enum Future1 {
    StateBegin,
    State1(Future2),
}

enum Future2 {
    StateBegin,
    State1(Future3),
}

struct Future3(i32);

impl Future for Future1 {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                Future1::StateBegin => {
                    println!("f1: begin");
                    *self = Future1::State1(Future2::StateBegin);
                }
                Future1::State1(f) => {
                    let mut f = pin!(f);
                    let r = ready!(f.as_mut().poll(cx));
                    println!("f1: end");
                    return Poll::Ready(r);
                }
            }
        }
    }
}

impl Future for Future2 {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                Future2::StateBegin => {
                    println!("f2: begin");
                    *self = Future2::State1(Future3(1));
                }
                Future2::State1(f) => {
                    let mut f = pin!(f);
                    let r = ready!(f.as_mut().poll(cx));
                    println!("f2: end");
                    return Poll::Ready(r);
                }
            }
        }
    }
}

impl Future for Future3 {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        println!("Future3.poll");
        if self.0 < 3 {
            self.0 += 1;
            Poll::Pending
        } else {
            Poll::Ready(self.0)
        }
    }
}
