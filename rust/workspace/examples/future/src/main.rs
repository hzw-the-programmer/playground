#![feature(noop_waker)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod drop;
mod future_async_await;
mod future_state_machine;
mod futures;
mod sink;
mod streams;

// use future_async_await as test;
// use future_state_machine as test;
// use sink as test;
// use futures as test;
use streams as test;
// use drop as test;

fn main() {
    test::test();
}
