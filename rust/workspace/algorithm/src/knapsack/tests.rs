use super::knapsack;

#[test]
fn test_knapsack() {
    assert_eq!(knapsack(&[1, 50], &[1, 30], 100), 100);
    assert_eq!(knapsack(&[1, 3, 4, 5], &[10, 40, 50, 70], 8), 110);
}

#[test]
fn test_capacity_zero() {
    assert_eq!(knapsack(&[2, 3], &[3, 4], 0), 0);
}

#[test]
fn test_single_item_fit() {
    assert_eq!(knapsack(&[5], &[30], 5), 30);
}

#[test]
fn test_single_item_too_heavy() {
    assert_eq!(knapsack(&[6], &[30], 5), 0);
}

#[test]
fn test_multiple_items() {
    assert_eq!(knapsack(&[2, 3], &[3, 4], 5), 7);
}

#[test]
fn test_large_capacity() {
    assert_eq!(knapsack(&[3, 5], &[50, 60], 10), 150);
}

#[test]
fn test_empty_items() {
    assert_eq!(knapsack(&[], &[], 10), 0);
}

#[test]
#[should_panic(expected = "Item with weight 0 and positive value is not allowed")]
fn test_invalid_item_zero_weight() {
    knapsack(&[0], &[5], 10);
}

#[test]
fn test_zero_value_item() {
    // 重量为0但价值也为0的物品应被忽略
    assert_eq!(knapsack(&[0, 2], &[0, 5], 4), 10); // 选两次2，总价值10
}
