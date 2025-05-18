use super::*;

#[test]
fn test_can_complete_circuit() {
    assert_eq!(
        can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}
