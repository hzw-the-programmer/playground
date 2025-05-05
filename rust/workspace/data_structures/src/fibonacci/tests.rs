use super::fibonacci;

#[test]
fn test_fibonacci() {
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (i, &ans) in expected.iter().enumerate() {
        assert_eq!(fibonacci(i), ans);
    }
}
