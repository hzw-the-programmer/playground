use super::trap;

#[test]
fn test_trap() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);

    assert_eq!(trap(vec![8, 6, 4, 5, 6, 7, 8]), 12);
}
