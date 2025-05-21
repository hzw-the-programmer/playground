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

    assert_eq!(candy(vec![10, 9, 8]), 6);
    assert_eq!(candy(vec![8, 9, 10]), 6);
    assert_eq!(candy(vec![8, 9, 10, 9]), 7);
    assert_eq!(candy(vec![8, 9, 10, 9, 8]), 9);
    assert_eq!(candy(vec![8, 9, 10, 9, 8, 7]), 13);
    assert_eq!(candy(vec![8, 9, 10, 9, 8, 7, 6]), 18);
    assert_eq!(candy(vec![8, 9, 10, 9, 8, 7, 6, 7]), 20);
    assert_eq!(candy(vec![8, 9, 10, 9, 8, 7, 6, 7, 6]), 21);

    assert_eq!(
        candy(vec![
            58, 21, 72, 77, 48, 9, 38, 71, 68, 77, 82, 47, 25, 94, 89, 54, 26, 54, 54, 99, 64, 71,
            76, 63, 81, 82, 60, 64, 29, 51, 87, 87, 72, 12, 16, 20, 21, 54, 43, 41, 83, 77, 41, 61,
            72, 82, 15, 50, 36, 69, 49, 53, 92, 77, 16, 73, 12, 28, 37, 41, 79, 25, 80, 3, 37, 48,
            23, 10, 55, 19, 51, 38, 96, 92, 99, 68, 75, 14, 18, 63, 35, 19, 68, 28, 49, 36, 53, 61,
            64, 91, 2, 43, 68, 34, 46, 57, 82, 22, 67, 89
        ]),
        208
    );
}
