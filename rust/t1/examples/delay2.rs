use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        println!("Delay: poll");
        if Instant::now() >= self.when {
            println!("Delay: Poll::Ready");
            Poll::Ready("done")
        } else {
            println!("Delay: Poll::Pending");
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use MainFuture::*;

        println!("MainFuture: poll *********");
        loop {
            println!("MainFuture: loop");
            match *self {
                State0 => {
                    println!("MainFuture: State0");
                    let when = Instant::now() + Duration::from_millis(1);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                    println!("MainFuture: State1");
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            println!("MainFuture: Poll::Ready");
                            assert_eq!(out, "done");
                            *self = Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            println!("MainFuture: Poll::Pending");
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}

use futures::executor::block_on;

fn main() {
    block_on(MainFuture::State0);
}
