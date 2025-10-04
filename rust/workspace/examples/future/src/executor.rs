use futures_executor::ThreadPool;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let pool = ThreadPool::builder()
        .after_start(|idx| {
            println!("{idx} after start");
        })
        .before_stop(|idx| {
            println!("{idx} before stop");
        })
        .create()
        .unwrap();

    thread::sleep(Duration::from_secs(3));
}

fn test2() {
    let pool = ThreadPool::builder()
        .after_start(|idx| {
            println!("{idx} after start");
        })
        .before_stop(|idx| {
            println!("{idx} before stop");
        })
        .pool_size(1)
        .create()
        .unwrap();

    pool.spawn_ok(Foo(0));

    thread::sleep(Duration::from_secs(3));
}

struct Foo(i32);

impl Future for Foo {
    type Output = ();

    fn poll(mut self: Pin<&mut Foo>, cx: &mut Context) -> Poll<()> {
        if self.0 == 0 {
            println!("Foo::poll Pending");
            self.0 += 1;
            let waker = cx.waker().clone();
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(1));
                waker.wake();
            });
            Poll::Pending
        } else {
            println!("Foo::poll Ready");
            Poll::Ready(())
        }
    }
}
