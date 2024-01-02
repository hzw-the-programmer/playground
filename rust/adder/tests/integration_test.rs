use adder;

// cargo test --test integration_test

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::adds_two(2));
}

mod common;

#[test]
fn another_test() {
    common::setup();
    assert_eq!(4, adder::adds_two(2));
}