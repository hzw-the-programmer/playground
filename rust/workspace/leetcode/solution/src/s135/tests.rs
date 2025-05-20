use super::*;

#[test]
fn test_candy() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
    assert_eq!(candy(vec![1, 2, 2]), 4);
    assert_eq!(candy(vec![1, 3, 2, 2, 1]), 7);
    assert_eq!(candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);

    assert_eq!(candy(vec![2, 2, 2]), 3);
    assert_eq!(candy(vec![1, 2, 3]), 6);
    assert_eq!(candy(vec![1, 2, 3, 2]), 7);
    assert_eq!(candy(vec![1, 2, 3, 2, 1]), 9);

    assert_eq!(candy(vec![1, 2, 3]), 6);
    assert_eq!(candy(vec![1, 2, 3, 2]), 7);
    assert_eq!(candy(vec![1, 2, 3, 2, 1]), 9);
    assert_eq!(candy(vec![1, 2, 3, 2, 1, 0]), 13);

    assert_eq!(candy(vec![3, 4, 5, 5]), 7);
    assert_eq!(candy(vec![3, 4, 5, 5, 5]), 8);
    assert_eq!(candy(vec![3, 4, 5, 5, 4]), 9);
    assert_eq!(candy(vec![3, 4, 5, 5, 5, 4]), 10);

    assert_eq!(candy(vec![3, 4, 5, 4, 4]), 8);
    assert_eq!(candy(vec![3, 4, 5, 4, 4, 4]), 9);
    assert_eq!(candy(vec![3, 4, 5, 4, 4, 4, 3]), 11);

    assert_eq!(candy(vec![4, 3, 3, 2, 1]), 9);
}
