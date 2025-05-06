use std::cmp::Ordering;

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    insertion_sort_by(arr, |a, b| a.cmp(b));
}

pub fn insertion_sort_by<T: Ord, F>(arr: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && f(&arr[j - 1], &arr[j]) == Ordering::Greater {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_unsorted() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut arr = vec![5, 3, 5, 3, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![3, 3, 5, 5, 5]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-5, 3, -2, 0, 7];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![-5, -2, 0, 3, 7]);
    }

    #[test]
    fn test_stable() {
        let mut arr = [(5, 5), (3, 2), (5, 3), (3, 4), (5, 1)];
        insertion_sort_by(&mut arr, |a, b| a.0.cmp(&b.0));
        assert_eq!(arr[0], (3, 2));
        assert_eq!(arr[1], (3, 4));
        assert_eq!(arr[2], (5, 5));
        assert_eq!(arr[3], (5, 3));
        assert_eq!(arr[4], (5, 1));
    }
}
