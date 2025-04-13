use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

pub fn test() {
    test1();
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
