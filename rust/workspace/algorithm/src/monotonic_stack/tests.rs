use super::*;

#[test]
fn test_next_greater_element() {
    assert_eq!(
        next_greater_element(&[2, 1, 2, 4, 3]),
        vec![4, 2, 4, -1, -1]
    );
}

#[test]
fn test_largest_rectangle_in_histogram() {
    assert_eq!(largest_rectangle_in_histogram(&[1, 2, 3, 7, 1]), 7);
    assert_eq!(largest_rectangle_in_histogram(&[1, 2, 3, 7, 7]), 14);
    assert_eq!(largest_rectangle_in_histogram(&[1, 2, 3, 1, 2, 3]), 6);

    assert_eq!(largest_rectangle_in_histogram(&[2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(
        largest_rectangle_in_histogram(&[60, 20, 50, 40, 10, 50, 60]),
        100
    );

    // important test
    assert_eq!(largest_rectangle_in_histogram(&[5, 4, 3, 2, 1]), 9);
    assert_eq!(largest_rectangle_in_histogram(&[2, 1, 2]), 3);
    assert_eq!(largest_rectangle_in_histogram(&[2, 4, 6, 3]), 9);
    assert_eq!(largest_rectangle_in_histogram(&[2, 5, 7, 4, 2]), 12);
}

#[test]
fn test_empty() {
    assert_eq!(largest_rectangle_in_histogram(&[]), 0);
}

#[test]
fn test_single_element() {
    assert_eq!(largest_rectangle_in_histogram(&[2]), 2);
}

#[test]
fn test_example() {
    assert_eq!(largest_rectangle_in_histogram(&[2, 1, 5, 6, 2, 3]), 10);
}

#[test]
fn test_increasing() {
    assert_eq!(largest_rectangle_in_histogram(&[1, 2, 3, 4, 5]), 9);
}

#[test]
fn test_2_4() {
    assert_eq!(largest_rectangle_in_histogram(&[2, 4]), 4);
}
