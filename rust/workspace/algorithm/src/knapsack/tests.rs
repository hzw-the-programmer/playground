use super::knapsack;

#[test]
fn test_knapsack() {
    assert_eq!(knapsack(&[1, 50], &[1, 30], 100), 100);
    assert_eq!(knapsack(&[1, 3, 4, 5], &[10, 40, 50, 70], 8), 110);
}
