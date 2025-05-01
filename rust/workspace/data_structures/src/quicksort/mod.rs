pub mod last;
// pub use last::quicksort;
pub mod strategy;
// pub use strategy::quicksort_last as quicksort;
// pub use strategy::quicksort_median_of_three as quicksort;
// pub use strategy::quicksort_random as quicksort;
pub use strategy::quicksort_hybrid as quicksort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        quicksort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![5]);
    }

    #[test]
    fn test_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_random() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut arr = vec![5, 3, 5, 3, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![3, 3, 5, 5, 5]);
    }
}
