use super::*;

#[test]
fn test_candy() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
    assert_eq!(candy(vec![1, 2, 2]), 4);

    assert_eq!(candy(vec![2, 2, 2]), 3);
    assert_eq!(candy(vec![1, 2, 3]), 6);
    assert_eq!(candy(vec![1, 2, 3, 2]), 7);
    assert_eq!(candy(vec![1, 2, 3, 2, 1]), 9);
}
