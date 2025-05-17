use super::*;

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    assert_eq!(max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);

    assert_eq!(max_profit(vec![1, 3, 2, 8, 9, 9], 2), 6);
    assert_eq!(max_profit(vec![1, 3, 2, 8, 8, 9], 2), 6);
    assert_eq!(max_profit(vec![1, 3, 2, 8, 7, 9], 2), 6);
    assert_eq!(max_profit(vec![1, 3, 2, 8, 6, 9], 2), 6);
    assert_eq!(max_profit(vec![1, 3, 2, 8, 5, 9], 2), 7);

    assert_eq!(max_profit(vec![9, 8, 7, 1, 2], 3), 0);
    assert_eq!(max_profit(vec![2, 2, 1, 1, 5, 5, 3, 1, 5, 4], 2), 4);
    assert_eq!(max_profit(vec![9, 1], 3), 0);
    assert_eq!(max_profit(vec![9, 8], 3), 0);
}
