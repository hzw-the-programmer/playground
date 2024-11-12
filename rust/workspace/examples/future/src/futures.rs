use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use futures_util::FutureExt;

pub fn test() {
    test1();
}

fn test1() {
    let mut cx = Context::from_waker(Waker::noop());

    // let fut = async { 1 };
    let fut = async { Foo(0).await };
    let mut fut = fut.map(|x| x + 3);
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(4));
}

struct Foo(i32);

impl Future for Foo {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.0 == 0 {
            self.0 += 1;
            Poll::Pending
        } else {
            Poll::Ready(self.0)
        }
    }
}
