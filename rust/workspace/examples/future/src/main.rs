// #![feature(noop_waker)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

mod drop;
mod future_2;
mod future_async_await;
mod future_state_machine;
mod futures;
mod service;
mod sink;
mod stream_2;
mod streams;

// use future_async_await as test;
// use future_state_machine as test;
// use sink as test;
// use futures as test;
// use streams as test;
// use drop as test;
// use future_2 as test;
// use stream_2 as test;
use service as test;

fn main() {
    test::test();
}
