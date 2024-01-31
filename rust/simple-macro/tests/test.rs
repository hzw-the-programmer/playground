use simple_macro::A;

#[derive(A)]
struct A;

#[test]
fn test_derive_a() {
    assert_eq!("hello from impl A".to_string(), A.a());
}
