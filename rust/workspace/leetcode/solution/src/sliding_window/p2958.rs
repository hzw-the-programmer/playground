// 2958. Length of Longest Subarray With at Most K Frequency

use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut map = HashMap::new();
    let mut l = 0;
    for r in 0..nums.len() {
        *map.entry(nums[r]).or_insert(0) += 1;
        while *map.get(&nums[r]).unwrap() > k {
            let c = map.get_mut(&nums[l]).unwrap();
            *c -= 1;
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2));
        assert_eq!(2, max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1));
        assert_eq!(4, max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4));
    }
}
