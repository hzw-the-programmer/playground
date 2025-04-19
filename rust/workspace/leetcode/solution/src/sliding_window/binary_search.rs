pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        // 避免整数溢出
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

pub fn lower_bound(arr: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(None, binary_search(&[], 1));
        assert_eq!(None, binary_search(&[2], 1));
        assert_eq!(None, binary_search(&[2, 4], 3));
        assert_eq!(None, binary_search(&[2, 4, 6], 3));
        assert_eq!(None, binary_search(&[2, 4, 6, 8], 7));
        assert_eq!(None, binary_search(&[2, 4, 6, 8, 10], 7));

        assert_eq!(Some(4), binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5));
        assert_eq!(Some(4), binary_search(&[1, 2, 3, 4, 5, 6, 7, 8], 5));
    }

    #[test]
    fn lower_bound_test() {
        assert_eq!(0, lower_bound(&[], 1));
        
        assert_eq!(0, lower_bound(&[2], 1));
        
        assert_eq!(1, lower_bound(&[2, 4], 3));
        
        assert_eq!(0, lower_bound(&[2, 4, 6], 1));
        assert_eq!(1, lower_bound(&[2, 4, 6], 3));
        assert_eq!(2, lower_bound(&[2, 4, 6], 5));
        assert_eq!(3, lower_bound(&[2, 4, 6], 7));
        
        assert_eq!(2, lower_bound(&[2, 4, 6, 8], 5));
        
        assert_eq!(0, lower_bound(&[2, 4, 6, 8, 10], 1));
        assert_eq!(1, lower_bound(&[2, 4, 6, 8, 10], 3));
        assert_eq!(2, lower_bound(&[2, 4, 6, 8, 10], 5));
        assert_eq!(3, lower_bound(&[2, 4, 6, 8, 10], 7));
        assert_eq!(4, lower_bound(&[2, 4, 6, 8, 10], 9));
        assert_eq!(5, lower_bound(&[2, 4, 6, 8, 10], 11));
        
        assert_eq!(0, lower_bound(&[2], 2));
        assert_eq!(0, lower_bound(&[2, 4], 2));
        assert_eq!(1, lower_bound(&[2, 4], 4));

        assert_eq!(0, lower_bound(&[2, 4, 6, 8, 10], 2));
        assert_eq!(1, lower_bound(&[2, 4, 6, 8, 10], 4));
        assert_eq!(2, lower_bound(&[2, 4, 6, 8, 10], 6));
        assert_eq!(3, lower_bound(&[2, 4, 6, 8, 10], 8));
        assert_eq!(4, lower_bound(&[2, 4, 6, 8, 10], 10));
    }
}
