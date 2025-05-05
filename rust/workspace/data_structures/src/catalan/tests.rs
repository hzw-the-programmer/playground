use super::catalan;

#[test]
fn test_catalan() {
    let expected = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for (i, &n) in expected.iter().enumerate() {
        assert_eq!(catalan(i), n);
    }
}
