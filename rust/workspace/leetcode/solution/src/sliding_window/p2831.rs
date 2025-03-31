// 2831. Find the Longest Equal Subarray

// 1 <= nums[i] <= nums.length
pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = vec![vec![]; nums.len() + 1];
    for (i, &n) in nums.iter().enumerate() {
        let pos_list = &mut map[n as usize];
        pos_list.push(i);
    }

    let mut res = 0;
    for pos_list in map {
        let mut left = 0;
        for right in 0..pos_list.len() {
            while pos_list[right] - pos_list[left] + 1 - (right - left + 1) > k as usize {
                left += 1;
            }
            res = res.max(right - left + 1);
        }
    }

    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3);
        assert_eq!(longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4);
        assert_eq!(longest_equal_subarray(vec![3, 1, 5, 3, 1, 1], 0), 2);
        assert_eq!(longest_equal_subarray(vec![4, 2, 7, 2, 1, 7, 2], 1), 2);
    }
}
