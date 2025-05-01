pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let p = partition(arr);
    let (left, right) = arr.split_at_mut(p);
    quicksort(left);
    quicksort(&mut right[1..]);
}

pub fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let high = arr.len() - 1;
    let mut i = 0;
    for j in 0..high {
        if arr[j] > arr[high] {
            continue;
        }
        arr.swap(i, j);
        i += 1;
    }
    arr.swap(i, high);
    i
}

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
