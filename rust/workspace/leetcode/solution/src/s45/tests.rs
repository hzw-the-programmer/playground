use super::*;

#[test]
fn test_jump() {
    assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    assert_eq!(jump(vec![0]), 0);
}
