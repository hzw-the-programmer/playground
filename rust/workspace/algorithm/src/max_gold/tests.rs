use super::max_gold;

#[test]
fn test_max_gold() {
    assert_eq!(max_gold(&[[1, 3, 3], [2, 1, 4], [0, 6, 4]]), 12);
    assert_eq!(
        max_gold(&[[1, 3, 1, 5], [2, 2, 4, 1], [5, 0, 2, 3], [0, 6, 1, 2]]),
        16
    );
}
