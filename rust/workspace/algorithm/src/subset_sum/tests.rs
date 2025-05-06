use super::subset_sum;

#[test]
fn test_subset_sum() {
    assert!(subset_sum(&[3, 34, 4, 12, 5, 2], 9));
    assert!(!subset_sum(&[3, 34, 4, 12, 5, 2], 30));
    assert!(subset_sum(&[0, 0, 0, 0, 0, 0], 0));
    assert!(!subset_sum(&[0, 0, 0, 0, 0, 0], 1));
}

#[test]
fn test_basic_case() {
    assert!(subset_sum(&[1, 2, 3, 4], 5));
}

#[test]
fn test_no_subset() {
    assert!(!subset_sum(&[2, 4, 6], 5));
}

#[test]
fn test_target_zero() {
    assert!(subset_sum(&[], 0));
    assert!(subset_sum(&[1, 2, 3], 0));
}

#[test]
fn test_exact_match() {
    assert!(subset_sum(&[5], 5));
    assert!(subset_sum(&[10, 20, 30], 30));
}

#[test]
fn test_combination() {
    assert!(subset_sum(&[3, 34, 4, 12, 5, 2], 9));
    assert!(!subset_sum(&[3, 34, 4, 12, 5, 2], 100));
}
