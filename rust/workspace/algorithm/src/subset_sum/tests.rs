use super::subset_sum;

#[test]
fn test_subset_sum() {
    assert!(subset_sum(&[3, 34, 4, 12, 5, 2], 9));
    assert!(!subset_sum(&[3, 34, 4, 12, 5, 2], 30));
}
