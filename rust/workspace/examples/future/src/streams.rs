use core::future::{poll_fn, ready, Future};
use core::pin::{pin, Pin};
use core::task::{ready, Context, Poll, Waker};
use futures_channel::{mpsc, oneshot};
use futures_executor as executor;
use futures_util::{future, select, stream, Sink, Stream, StreamExt};
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
    // any();
    // all();
    // flatten();
    // flatten_unordered();
    // flat_map();
    // flat_map_unordered();
    // scan();
    // skip_while();
    // take_while();
    // take_until();
    // for_each();
    // for_each_concurrent();
    // take();
    // skip();
    // fuse();
    // by_ref();
    // catch_unwind();
    // buffered();
    // buffered_1();
    // buffer_unordered();
    // zip();
    // chain();
    // peek();
    // chunks();
    // ready_chunks();
    // forward_1();
    // inspect();
    select_next_some();
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

fn all() {
    executor::block_on(async {
        let st = stream::iter(1..10);
        let fut = st.any(|i| ready(i < 20));
        assert_eq!(fut.await, true);
    });
}

fn flatten() {
    executor::block_on(async {
        let (tx1, rx1) = mpsc::unbounded();
        let (tx2, rx2) = mpsc::unbounded();
        let (tx3, rx3) = mpsc::unbounded();

        thread::spawn(move || {
            tx1.unbounded_send(1).unwrap();
            tx1.unbounded_send(2).unwrap();
        });
        thread::spawn(move || {
            tx2.unbounded_send(3).unwrap();
            tx2.unbounded_send(4).unwrap();
        });
        thread::spawn(move || {
            tx3.unbounded_send(rx1).unwrap();
            tx3.unbounded_send(rx2).unwrap();
        });

        let fut = rx3.flatten().collect::<Vec<_>>();
        assert_eq!(fut.await, vec![1, 2, 3, 4]);
    });
}

fn flatten_unordered() {
    executor::block_on(async {
        let (t1, r1) = mpsc::unbounded();
        let (t2, r2) = mpsc::unbounded();
        let (t3, r3) = mpsc::unbounded();

        thread::spawn(move || {
            t1.unbounded_send(1).unwrap();
            t1.unbounded_send(2).unwrap();
        });

        thread::spawn(move || {
            t2.unbounded_send(3).unwrap();
            t2.unbounded_send(4).unwrap();
        });

        thread::spawn(move || {
            t3.unbounded_send(r1).unwrap();
            t3.unbounded_send(r2).unwrap();
        });

        let fut = r3.flatten_unordered(0).collect::<Vec<_>>();
        let mut v = fut.await;
        println!("{:?}", v);
        v.sort();
        assert_eq!(v, vec![1, 2, 3, 4]);
    });
}

fn flat_map() {
    executor::block_on(async {
        let st = stream::iter(1..=3);
        let st = st.flat_map(|i| stream::iter(vec![i + 3; i]));
        assert_eq!(vec![4, 5, 5, 6, 6, 6], st.collect::<Vec<_>>().await);
    });
}

fn flat_map_unordered() {
    executor::block_on(async {
        let st = stream::iter(1..5);
        let st = st.flat_map_unordered(0, |i| stream::iter(vec![i; i]));
        let mut values = st.collect::<Vec<_>>().await;
        println!("{:?}", values);
        values.sort();
        assert_eq!(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], values);
    });
}

fn scan() {
    executor::block_on(async {
        let st = stream::iter(1..10);
        let st = st.scan(0, |state, i| {
            *state += i;
            ready(if *state < 10 { Some(i) } else { None })
        });
        assert_eq!(vec![1, 2, 3], st.collect::<Vec<_>>().await);
    });
}

fn skip_while() {
    executor::block_on(async {
        let st = stream::iter(1..10);
        let st = st.skip_while(|i| ready(*i < 6));
        assert_eq!(vec![6, 7, 8, 9], st.collect::<Vec<_>>().await);
    });
}

fn take_while() {
    executor::block_on(async {
        let st = stream::iter(1..10);
        let st = st.take_while(|i| ready(*i < 6));
        assert_eq!(vec![1, 2, 3, 4, 5], st.collect::<Vec<_>>().await);
    });
}

fn take_until() {
    executor::block_on(async {
        let mut i = 0;
        let stop_fut = poll_fn(|_cx| {
            i += 1;
            if i < 5 {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        });
        let st = stream::iter(1..10);
        let st = st.take_until(stop_fut);
        assert_eq!(vec![1, 2, 3, 4], st.collect::<Vec<_>>().await);
    });
}

fn for_each() {
    executor::block_on(async {
        let mut x = 0;
        {
            let fut = stream::repeat(1).take(3).for_each(|i| {
                x += i;
                ready(())
            });
            fut.await;
        }
        assert_eq!(3, x);
    });
}

fn for_each_concurrent() {
    executor::block_on(async {
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();
        let fut = stream::iter(vec![rx1, rx2, rx3]).for_each_concurrent(0, |rx| async move {
            rx.await.unwrap();
        });
        tx1.send(()).unwrap();
        tx2.send(()).unwrap();
        tx3.send(()).unwrap();
        fut.await;
    });
}

fn take() {
    executor::block_on(async {
        let st = stream::iter(1..10).take(3);
        assert_eq!(vec![1, 2, 3], st.collect::<Vec<_>>().await);
    });
}

fn skip() {
    executor::block_on(async {
        let st = stream::iter(1..10).skip(7);
        assert_eq!(vec![8, 9], st.collect::<Vec<_>>().await);
    });
}

fn fuse() {
    let mut x = 0;
    let st = stream::poll_fn(|_| {
        x += 1;
        match x {
            0..=2 => Poll::Ready(Some(x)),
            3 => Poll::Ready(None),
            _ => panic!("should not happen"),
        }
    });
    // let mut iter = executor::block_on_stream(st);
    let mut iter = executor::block_on_stream(st.fuse());
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
}

fn by_ref() {
    executor::block_on(async {
        let mut st = stream::iter(1..=5);
        let sum = st
            .by_ref()
            .take(2)
            .fold(0, |a, b| async move { a + b })
            .await;
        assert_eq!(3, sum);
        let sum = st.take(2).fold(0, |a, b| async move { a + b }).await;
        assert_eq!(7, sum);
    });
}

fn catch_unwind() {
    executor::block_on(async {
        let st = stream::iter(vec![Some(10), None, Some(12)]);
        let st = st.map(|o| o.unwrap());
        let st = st.catch_unwind();
        let results = st.collect::<Vec<_>>().await;
        assert_eq!(results.len(), 2);
        match results[0] {
            Ok(i) => println!("{}", i),
            _ => panic!("unexpected"),
        }
        assert!(results[1].is_err());
    });
}

fn buffered() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let st = st.map(|i| {
            println!("return fut {}", i);
            async move {
                println!("poll fut {}", i);
                i
            }
        });
        let st = st.buffered(3);
        let r = st.collect::<Vec<_>>().await;
        println!("{:?}", r);
    });
}

fn buffered_1() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let st = st.map(|i| {
            println!("return fut {}", i);
            let mut j = i;
            poll_fn(move |cx| {
                println!("poll fut {}", i);
                if j % 2 == 0 {
                    j += 1;
                    let w = cx.waker().clone();
                    thread::spawn(move || w.wake());
                    Poll::Pending
                } else {
                    Poll::Ready(i)
                }
            })
        });
        let st = st.buffered(3);
        let r = st.collect::<Vec<_>>().await;
        println!("{:?}", r);
    });
}

fn buffer_unordered() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let st = st.map(|i| {
            println!("return fut {}", i);
            let mut j = i;
            poll_fn(move |cx| {
                println!("poll fut {}", i);
                if j % 2 == 0 {
                    j += 1;
                    let w = cx.waker().clone();
                    thread::spawn(move || w.wake());
                    Poll::Pending
                } else {
                    Poll::Ready(i)
                }
            })
        });
        let st = st.buffer_unordered(3);
        let r = st.collect::<Vec<_>>().await;
        println!("{:?}", r);
    });
}

fn zip() {
    executor::block_on(async {
        let st1 = stream::iter(1..=3);
        let st2 = stream::iter(5..=10);
        let st = st1.zip(st2);
        let results = st.collect::<Vec<_>>().await;
        println!("{:?}", results);
    });
}

fn chain() {
    executor::block_on(async {
        let st1 = stream::iter(vec![Ok(10), Err(false)]);
        let st2 = stream::iter(vec![Err(true), Ok(20)]);
        let st = st1.chain(st2);
        let r: Vec<_> = st.collect().await;
        assert_eq!(vec![Ok(10), Err(false), Err(true), Ok(20)], r);
    });
}

fn peek() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let mut st = pin!(st.peekable());
        assert_eq!(Some(&1), st.as_mut().peek().await);
        assert_eq!(Some(&1), st.as_mut().peek().await);
        assert_eq!(Some(1), st.next().await);
        assert_eq!(Some(2), st.next().await);
        assert_eq!(Some(&3), st.as_mut().peek().await);
        assert_eq!(Some(&3), st.as_mut().peek().await);
        assert_eq!(Some(3), st.next().await);
        assert_eq!(Some(&mut 4), st.as_mut().peek_mut().await);
        assert_eq!(Some(&mut 4), st.as_mut().peek_mut().await);
        assert_eq!(Some(4), st.next().await);
        assert_eq!(Some(5), st.as_mut().next_if(|&x| x < 6).await);
        assert_eq!(None, st.as_mut().next_if(|&x| x < 6).await);
        assert_eq!(None, st.as_mut().next_if(|&x| x < 6).await);
        assert_eq!(Some(&mut 6), st.as_mut().peek_mut().await);
        assert_eq!(Some(6), st.next().await);
        while st.as_mut().next_if(|&x| x < 9).await.is_some() {}
        assert_eq!(Some(&9), st.as_mut().peek().await);
        assert_eq!(Some(&mut 9), st.as_mut().peek_mut().await);
    });
}

fn chunks() {
    executor::block_on(async {
        let st = stream::iter(1..=10);
        let mut st = st.chunks(3);
        assert_eq!(Some(vec![1, 2, 3]), st.next().await);
        assert_eq!(Some(vec![4, 5, 6]), st.next().await);
        assert_eq!(Some(vec![7, 8, 9]), st.next().await);
        assert_eq!(Some(vec![10]), st.next().await);
        assert_eq!(None, st.next().await);
    });
}

fn ready_chunks() {
    executor::block_on(async {
        let mut counter = 0;
        let st = stream::poll_fn(|_| {
            counter += 1;
            if counter % 3 == 0 && counter != 9 {
                Poll::Pending
            } else {
                Poll::Ready(Some(counter))
            }
        });
        let mut st = st.ready_chunks(3);
        assert_eq!(Some(vec![1, 2]), st.next().await);
        assert_eq!(Some(vec![4, 5]), st.next().await);
        assert_eq!(Some(vec![7, 8, 9]), st.next().await);
    });
}

fn forward_1() {
    executor::block_on(async {
        let st = stream::iter((1..=10).map(Ok));
        let _ = st.forward(Foo(0)).await;
    });
}

fn inspect() {
    executor::block_on(async {
        let st = stream::iter(1..=3);
        let st = st.inspect(|x| {
            // let i: i32 = x;
            println!("{:?}", x);
        });
        assert_eq!(vec![1, 2, 3], st.collect::<Vec<_>>().await);
    });
}

fn select_next_some() {
    executor::block_on(async {
        let mut fut = future::ready(1);
        let mut tasks = stream::FuturesUnordered::new();
        let mut total = 0;
        loop {
            select! {
                num = fut => {
                    total += num;
                    tasks.push(async {5});
                }
                num = tasks.select_next_some() => total += num,
                complete => break,
            }
        }
        assert_eq!(6, total);
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

impl<Item> Sink<Item> for Foo {
    type Error = ();

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        println!("Foo::poll_ready");
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        println!("Foo::start_send");
        Ok(())
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        println!("Foo::poll_close");
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        println!("Foo::poll_flush");
        Poll::Ready(Ok(()))
    }
}
