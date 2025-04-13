use std::fmt::Debug;
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};
use tower::{Service, ServiceExt};

pub fn test() {
    test1();
}

fn test1() {
    let mut cx = Context::from_waker(Waker::noop());
    let srv = Srv::new(2, 3);
    let mut fut = pin!(srv.oneshot(23));

    let r = fut.as_mut().poll(&mut cx);
    println!("{r:?}");

    let r = fut.as_mut().poll(&mut cx);
    println!("{r:?}");

    let r = fut.as_mut().poll(&mut cx);
    println!("{r:?}");

    let r = fut.as_mut().poll(&mut cx);
    println!("{r:?}");

    let r = fut.as_mut().poll(&mut cx);
    println!("{r:?}");

    let r = fut.as_mut().poll(&mut cx);
    println!("{r:?}");

    // thread 'main' panicked at D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\tower-0.5.2\src\util\oneshot.rs:101:36:
    // polled after complete
    // let r = fut.as_mut().poll(&mut cx);
    // println!("{r:?}");
}

struct Srv {
    cnt1: usize,
    cnt2: usize,
}

impl Srv {
    fn new(cnt1: usize, cnt2: usize) -> Self {
        Srv { cnt1, cnt2 }
    }
}

impl<Req: Debug> Service<Req> for Srv {
    type Response = i32;
    type Error = i32;
    type Future = Fut;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        println!("Srv::poll_ready: {}", self.cnt1);
        if self.cnt1 > 0 {
            self.cnt1 -= 1;
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn call(&mut self, req: Req) -> Self::Future {
        println!("Srv::call: req={req:?}");
        Fut(self.cnt2)
    }
}

struct Fut(usize);

impl Future for Fut {
    type Output = Result<i32, i32>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("Fut::poll: {}", self.0);
        if self.0 > 0 {
            self.0 -= 1;
            Poll::Pending
        } else {
            Poll::Ready(Ok(23))
        }
    }
}
