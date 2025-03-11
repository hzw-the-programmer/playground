pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        // 避免整数溢出
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            // 避免索引越界
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(None, binary_search(&[2], 1));
        assert_eq!(Some(4), binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5));
        assert_eq!(Some(4), binary_search(&[1, 2, 3, 4, 5, 6, 7, 8], 5));
    }
}
