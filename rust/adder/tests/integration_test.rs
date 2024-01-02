use adder;

// cargo test --test integration_test

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::adds_two(2));
}