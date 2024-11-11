use core::future::Future;
use core::pin::{pin, Pin};
use core::task::{Context, Poll, Waker};
use futures_core::Stream;
use futures_sink::Sink;
use futures_util::sink::SinkExt;

pub fn test() {
    test1();
}

fn test1() {
    let mut foo = Foo(0);
    let mut bar = Bar(0);
    let fut = foo.send_all(&mut bar);

    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = pin!(fut);

    let r = fut.as_mut().poll(&mut cx);
    println!("{:?}", r);

    let r = fut.as_mut().poll(&mut cx);
    println!("{:?}", r);

    let r = fut.as_mut().poll(&mut cx);
    println!("{:?}", r);
}

struct Foo(i32);

impl<Item: std::fmt::Debug> Sink<Item> for Foo {
    type Error = ();

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Foo::poll_ready: {:?}", self.0);
        if self.0 % 2 == 0 {
            self.0 += 1;
            Poll::Pending
        } else {
            self.0 += 1;
            Poll::Ready(Ok(()))
        }
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        println!("Foo::start_send: {:?}", item);
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Foo::poll_flush");
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Foo::poll_close");
        Poll::Ready(Ok(()))
    }
}

struct Bar(i32);

impl Stream for Bar {
    type Item = Result<i32, ()>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("Bar::poll_next");

        if self.0 < 2 {
            Poll::Ready(Some(Ok({
                let r = self.0;
                self.0 += 1;
                r
            })))
        } else {
            Poll::Ready(None)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        println!("Bar::size_hint");
        (0, None)
    }
}
