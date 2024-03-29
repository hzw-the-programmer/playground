use simple_macro::{attr_with_args, hashmap, New, A};

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

#[test]
fn test_hashmap() {
    let hm = hashmap!("a": 1, "b": 2,);
    assert_eq!(hm["a"], 1);
}

#[derive(New, Debug, PartialEq)]
struct Foo {
    id: i32,
}

#[test]
fn test_foo_new() {
    let f = Foo::new();
    assert_eq!(f, Foo { id: 1 });
}
