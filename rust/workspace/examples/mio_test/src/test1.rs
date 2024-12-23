use core::time::Duration;
use mio::{Events, Poll, Token, Waker};
use std::sync::Arc;
use std::thread;

pub fn test() {
    test1();
}

fn test1() {
    const WAKER_TOKEN: Token = Token(10);

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(2);

    let waker = Arc::new(Waker::new(poll.registry(), WAKER_TOKEN).unwrap());

    let waker2 = waker.clone();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        waker2.wake().unwrap();
        thread::sleep(Duration::from_secs(2));
    });

    println!("before poll");
    poll.poll(&mut events, None).unwrap();
    println!("after poll");
    assert!(!events.is_empty());
    let e = events.iter().next().unwrap();
    assert_eq!(e.token(), WAKER_TOKEN);
    assert!(e.is_readable());

    println!("before join");
    handle.join().unwrap();
    println!("after join");
}
