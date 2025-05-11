use super::combine;

#[test]
fn test_combine() {
    assert_eq!(
        combine(4, 2),
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ]
    );
    assert_eq!(combine(1, 1), vec![vec![1]]);
}
