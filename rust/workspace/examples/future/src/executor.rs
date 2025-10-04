use futures_executor::ThreadPool;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

pub fn test() {
    // test1();
    // test2();
    // thread_pool_3();
    local_pool_1();
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

fn thread_pool_3() {
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

    pool.spawn_ok(async {
        println!("async block begin");
        Foo(0).await;
        println!("Foo(0).await return");
        Bar(0).await;
        println!("async block end");
    });

    thread::sleep(Duration::from_secs(3));
}

fn local_pool_1() {
    futures_executor::block_on(async {
        println!("async block begin");
        Foo(0).await;
        println!("Foo(0).await return");
        Bar(0).await;
        println!("async block end");
    });
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
                println!("wake");
                waker.wake();
            });
            Poll::Pending
        } else {
            println!("Foo::poll Ready");
            Poll::Ready(())
        }
    }
}

struct Bar(i32);

impl Future for Bar {
    type Output = ();

    fn poll(mut self: Pin<&mut Bar>, cx: &mut Context) -> Poll<()> {
        if self.0 == 0 {
            println!("Bar::poll Pending");
            self.0 += 1;
            let waker = cx.waker().clone();
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(1));
                println!("wake");
                waker.wake();
            });
            Poll::Pending
        } else {
            println!("Bar::poll Ready");
            Poll::Ready(())
        }
    }
}
