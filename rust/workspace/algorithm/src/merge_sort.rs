pub fn merge_sort<T: Ord + Copy>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);
    merge(&left, &right)
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_element() {
        let arr = vec![5];
        assert_eq!(merge_sort(&arr), vec![5]);
    }

    #[test]
    fn test_sorted_array() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_unsorted() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(merge_sort(&arr), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_with_duplicates() {
        let arr = vec![2, 2, 2, 2];
        assert_eq!(merge_sort(&arr), vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_stable() {
        use std::cmp::Ordering;

        #[derive(Copy, Clone, Debug)]
        struct Foo {
            id: usize,
            weight: i32,
        }
        impl PartialEq for Foo {
            fn eq(&self, other: &Self) -> bool {
                self.weight == other.weight
            }
        }
        impl PartialOrd for Foo {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.weight.partial_cmp(&other.weight)
            }
        }
        impl Eq for Foo {}
        impl Ord for Foo {
            fn cmp(&self, other: &Self) -> Ordering {
                self.weight.cmp(&other.weight)
            }
        }

        let weights = [1, 2, 3, 1, 2, 3];
        let mut foos = Vec::with_capacity(weights.len());
        for (i, &w) in weights.iter().enumerate() {
            foos.push(Foo { id: i, weight: w });
        }
        let result = merge_sort(&foos);
        foos.sort();
        result
            .iter()
            .zip(foos.iter())
            .for_each(|(r, f)| assert_eq!(r.id, f.id));
    }
}
