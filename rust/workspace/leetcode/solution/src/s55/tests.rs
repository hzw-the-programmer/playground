use super::*;

#[test]
fn test_can_jump() {
    assert!(can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!can_jump(vec![3, 2, 1, 0, 4]));
}
