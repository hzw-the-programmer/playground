use core::future::Future;
use core::task::{Context, Poll, Waker};
use std::pin::{pin, Pin};

pub fn test() {
    // test1();
    // test2();
    test3();
}

fn test1() {
    println!("\ntest1: begin");

    let mut cx = Context::from_waker(Waker::noop());

    // no method named `poll` found for opaque type `impl Future<Output = ()>` in the current scope
    // method `poll` found on `Pin<&mut impl Future<Output = ()>>`
    // f1().poll(&mut cx);

    let mut pinned = pin!(f1());
    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 1: {r:?}");
    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 2: {r:?}");
    let r = pinned.as_mut().poll(&mut cx);
    println!("after poll 3: {r:?}");

    // let mut future = Box::pin(f1());
    // future.as_mut().poll(&mut cx);

    println!("test1: end");
}

fn test2() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut f = pin!(f3());

    let r = f.as_mut().poll(&mut cx);
    println!("poll 1: {r:?}");

    let r = f.as_mut().poll(&mut cx);
    println!("poll 2: {r:?}");
}

async fn f1() -> i32 {
    println!("f1: begin");

    let r = f2().await;

    println!("f1: end");

    r
}

async fn f2() -> i32 {
    println!("f2: begin");

    let r = Foo(1).await;

    println!("f2: end");

    r
}

struct Foo(i32);

impl Future for Foo {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        println!("Foo.poll");
        if self.0 < 3 {
            self.0 += 1;
            Poll::Pending
        } else {
            Poll::Ready(self.0)
        }
    }
}

struct Baz(i32);

impl Drop for Baz {
    fn drop(&mut self) {
        println!("Baz {} drop", self.0);
    }
}

async fn f3() -> i32 {
    println!("f3: entered");
    let baz = Baz(1);

    let res = Bar(0).await;
    println!("f3: {res:?}");

    3
}

struct Bar(i32);

impl Future for Bar {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<i32> {
        if self.0 == 1 {
            println!("Bar ready");
            Poll::Ready(4)
        } else {
            self.0 += 1;
            println!("Bar pending");
            Poll::Pending
        }
    }
}

fn test3() {
    fn f<T: Future>(f: T) {
        println!("{}", std::mem::size_of::<T>());
    }

    // 8
    f(async {
        Bar(1).await;
    });

    // 8
    f(async {
        let buf = [0; 1024];
        Bar(1).await;
    });

    // 1032 = 1024 + 8
    f(async {
        let buf = [0u8; 1024];
        Bar(1).await;
        let r = buf[0];
    });

    // 8
    f(async {
        struct Foo([u8; 1024]);
        let foo = Foo([0; 1024]);
        Bar(1).await;
    });

    // 1032 = 1024 + 8
    f(async {
        struct Foo([u8; 1024]);
        impl Drop for Foo {
            fn drop(&mut self) {
                println!("hhhh");
            }
        }
        let foo = Foo([0; 1024]);
        Bar(1).await;
    })
}
