use simple_macro::{attr_with_args, A};

#[derive(A)]
struct A;

#[test]
fn test_derive_a() {
    assert_eq!("hello from impl A".to_string(), A.a());
}

#[attr_with_args("Hello Rust!")]
fn foo() {
    println!("hhh");
}

#[test]
fn test_foo() {
    assert_eq!("Hello Rust!", foo());
}
