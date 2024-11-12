#![feature(noop_waker)]

mod future_async_await;
mod future_state_machine;
mod sink;

// use future_async_await as test;
use future_state_machine as test;
// use sink as test;

fn main() {
    test::test();
}
