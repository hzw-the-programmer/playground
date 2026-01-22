use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::runtime::{Builder, Handle};
use tokio::task;
// RUSTFLAGS="--cfg tokio_unstable" cargo run

pub fn test() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    // test8();
    test9();
}

fn test1() {
    let rt = Builder::new_current_thread().build().unwrap();
    let metrics = rt.metrics();
    println!("{}", metrics.remote_schedule_count());
    rt.spawn(async {});
    println!("{}", metrics.remote_schedule_count());
}

fn test2() {
    // there is no reactor running, must be called from the context of a Tokio 1.x runtime
    let handle = Handle::current();
}

fn test3() {
    let rt = Builder::new_current_thread().build().unwrap();
    // there is no reactor running, must be called from the context of a Tokio 1.x runtime
    let handle = Handle::current();
}

fn test4() {
    let rt = Builder::new_current_thread().build().unwrap();
    let _guard = rt.enter();
    let handle = Handle::current();
    rt.spawn(async {});
    let metrics = rt.metrics();
    println!("{}", metrics.remote_schedule_count());
}

fn test5() {
    let rt = Builder::new_current_thread().build().unwrap();
    rt.block_on(async {
        println!("root future begin");
        task::spawn(async {
            println!("child 1");
        });
        task::spawn(async {
            println!("child 2");
        });
    });
}

fn test6() {
    let rt = Builder::new_current_thread().build().unwrap();
    rt.block_on(async {
        println!("root future begin");
        task::spawn(async {
            println!("child 1");
        })
        .await
        .unwrap();
        task::spawn(async {
            println!("child 2");
        });
    });
}

fn test7() {
    let rt = Builder::new_current_thread()
        .on_thread_park(|| println!("before_park"))
        .on_thread_unpark(|| println!("after_unpark"))
        .build()
        .unwrap();
    rt.block_on(async {
        println!("root future begin");
        let j1 = task::spawn(async {
            println!("child 1");
        });
        task::spawn(async {
            println!("child 2");
        });
        j1.await.unwrap();
    });
}

fn test8() {
    let rt = Builder::new_current_thread()
        .on_thread_park(|| println!("before_park"))
        .on_thread_unpark(|| println!("after_unpark"))
        .build()
        .unwrap();
    rt.block_on(async {
        println!("root future begin");
        task::spawn(async {
            println!("child 1");
        });
        task::spawn(async {
            println!("child 2");
        });
        Foo(0).await;
    });
}

struct Foo(i32);

impl Future for Foo {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.0 == 0 {
            let waker = cx.waker().clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(3));
                waker.wake();
            });
            self.0 = 1;
            Poll::Pending
        } else {
            println!("Foo ready");
            Poll::Ready(())
        }
    }
}

fn test9() {
    let rt = Builder::new_current_thread()
        .enable_time()
        .on_thread_park(|| println!("before_park"))
        .on_thread_unpark(|| println!("after_unpark"))
        .build()
        .unwrap();
    rt.block_on(async {
        println!("root future begin");
        task::spawn(async {
            println!("child 1");
        });
        task::spawn(async {
            println!("child 2");
        });
        tokio::time::sleep(Duration::from_secs(3)).await;
    });
}
