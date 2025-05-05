use super::count_ways;

#[test]
fn test_count_ways() {
    let expected = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (i, &ans) in expected.iter().enumerate() {
        assert_eq!(count_ways(i), ans);
    }
}
