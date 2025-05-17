use super::*;

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(max_profit(vec![2, 1, 2, 0, 1]), 2);
}
