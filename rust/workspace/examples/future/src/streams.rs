use core::future::{ready, Future};
use core::pin::{pin, Pin};
use core::task::{ready, Context, Poll, Waker};
use futures_channel::mpsc;
use futures_executor as executor;
use futures_util::{stream, Stream, StreamExt};
use std::thread;

pub fn test() {
    // next_1();
    // next_2();
    // let _ = into_future();
    // map();
    // enumerate();
    // filter();
    // filter_map();
    // then();
    // unzip();
    // concat();
    // count();
    // cycle();
    // fold();
    any();
}

fn next_1() {
    let mut st = stream::iter(1..=3);
    let mut fut = pin!(st.next());

    let mut cx = Context::from_waker(Waker::noop());

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

fn next_2() {
    let mut st = Foo(0);
    // let mut fut = pin!(st.next());

    let mut cx = Context::from_waker(Waker::noop());

    println!("poll begin");
    // let r = fut.as_mut().poll(&mut cx);
    let r = pin!(st.next()).poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    // let r = fut.as_mut().poll(&mut cx);
    let r = pin!(st.next()).poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    // let r = fut.as_mut().poll(&mut cx);
    let r = pin!(st.next()).poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    // let r = fut.as_mut().poll(&mut cx);
    let r = pin!(st.next()).poll(&mut cx);
    println!("poll end: {:?}\n", r);

    println!("poll begin");
    // let r = fut.as_mut().poll(&mut cx);
    let r = pin!(st.next()).poll(&mut cx);
    println!("poll end: {:?}\n", r);
}

fn into_future() -> Poll<()> {
    let mut cx = Context::from_waker(Waker::noop());

    let st = Foo(0);

    let mut fut = pin!(st.into_future());
    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);
    println!("poll begin");
    let r = fut.as_mut().poll(&mut cx);
    println!("poll end: {:?}\n", r);

    let r = ready!(r);
    let fut = pin!(r.1.into_future());
    println!("poll begin");
    let r = fut.poll(&mut cx);
    println!("poll end: {:?}\n", r);

    let r = ready!(r);
    let fut = pin!(r.1.into_future());
    println!("poll begin");
    let r = fut.poll(&mut cx);
    println!("poll end: {:?}\n", r);

    Poll::Ready(())
}

fn map() {
    executor::block_on(async {
        let st = stream::iter(1..=3);
        let st = st.map(|i| i + 1);
        assert_eq!(vec![2, 3, 4], st.collect::<Vec<_>>().await);
    });
}

fn enumerate() {
    executor::block_on(async {
        let st = stream::iter(vec!['a', 'b', 'c']);
        let mut st = st.enumerate();
        assert_eq!(Some((0, 'a')), st.next().await);
        assert_eq!(Some((1, 'b')), st.next().await);
        assert_eq!(Some((2, 'c')), st.next().await);
        assert_eq!(None, st.next().await);
    });
}

fn filter() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let st = st.filter(|i| ready(i % 2 == 0));
        assert_eq!(vec![2, 4, 6, 8, 10], st.collect::<Vec<_>>().await);
    });
}

fn filter_map() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let st = st.filter_map(|i| async move {
            if i % 2 == 0 {
                Some(i + 1)
            } else {
                None
            }
        });
        assert_eq!(vec![3, 5, 7, 9, 11], st.collect::<Vec<_>>().await);
    });
}

fn then() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let st = st.then(|i| async move { i + 1 });
        assert_eq!(
            vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            st.collect::<Vec<_>>().await
        );
    });
}

fn unzip() {
    executor::block_on(async {
        let (tx, rx) = mpsc::unbounded();

        thread::spawn(move || {
            tx.unbounded_send(('a', 1)).unwrap();
            tx.unbounded_send(('b', 2)).unwrap();
            tx.unbounded_send(('c', 3)).unwrap();
        });

        let (a, b): (Vec<_>, Vec<_>) = rx.unzip().await;

        assert_eq!(vec!['a', 'b', 'c'], a);
        assert_eq!(vec![1, 2, 3], b);
    });
}

fn concat() {
    executor::block_on(async {
        let (tx, rx) = mpsc::unbounded();

        thread::spawn(move || {
            for i in (1..3).rev() {
                let n = i * 3;
                tx.unbounded_send(vec![n + 1, n + 2, n + 3]).unwrap();
            }
        });

        let r = rx.concat().await;

        assert_eq!(vec![7, 8, 9, 4, 5, 6], r);
    });
}

fn count() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        assert_eq!(10, st.count().await);
    });
}

fn cycle() {
    executor::block_on(async {
        let a = [1, 2, 3];
        let st = stream::iter(a);
        let mut st = st.cycle();
        assert_eq!(Some(1), st.next().await);
        assert_eq!(Some(2), st.next().await);
        assert_eq!(Some(3), st.next().await);
        assert_eq!(Some(1), st.next().await);
        assert_eq!(Some(2), st.next().await);
        assert_eq!(Some(3), st.next().await);
    });
}

fn fold() {
    executor::block_on(async {
        let st = stream::iter(1..6);
        let sum = st.fold(0, |a, e| ready(a + e));
        assert_eq!(sum.await, 15);
    });
}

fn any() {
    executor::block_on(async {
        let st = stream::iter(1..10);
        let fut = st.any(|i| ready(i % 3 == 0));
        assert_eq!(fut.await, true);
    });
}

#[derive(Debug)]
struct Foo(i32);

impl Stream for Foo {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let r = self.0;
        self.0 += 1;
        if r < 2 {
            println!("Foo::poll_next -> Pending");
            Poll::Pending
        } else if r < 4 {
            println!("Foo::poll_next -> Ready(Some({}))", r);
            Poll::Ready(Some(r))
        } else {
            println!("Foo::poll_next -> Ready(None)");
            Poll::Ready(None)
        }
    }
}
