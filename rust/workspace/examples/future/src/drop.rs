use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{Context, Poll, Waker};

pub fn test() {
    test1();
}

fn test1() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = pin!(async {
        println!("1");
        let i = Foo(0, 0).await;
        println!("Foo(0, 0) finish: {}", i);
        Foo(1, 1).await;
        println!("Foo(1, 1) finish: {}", i);
    });

    println!("before poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("after poll: {:?}", r);

    println!("\nbefore poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("after poll: {:?}", r);

    println!("\nbefore poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("after poll: {:?}", r);

    println!("\nbefore poll");
    let r = fut.as_mut().poll(&mut cx);
    println!("after poll: {:?}", r);
}

struct Foo(i32, i32);

impl Future for Foo {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let i = self.0;
        self.0 += 1;
        if i < 2 {
            println!("Foo::poll: {} -> Pending", self.1);
            Poll::Pending
        } else {
            println!("Foo::poll: {} -> Poll::Ready({})", self.1, i);
            Poll::Ready(i)
        }
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo::drop: {}", self.1);
    }
}
