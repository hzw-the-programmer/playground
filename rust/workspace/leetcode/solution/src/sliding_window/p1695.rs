// 1695. Maximum Erasure Value

use std::collections::HashSet;

pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut sum = 0;
    let mut res = 0;
    let mut l = 0;
    for r in 0..nums.len() {
        while set.contains(&nums[r]) {
            set.remove(&nums[l]);
            sum -= nums[l];
            l += 1;
        }
        set.insert(nums[r]);
        sum += nums[r];
        res = res.max(sum);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(17, maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
        assert_eq!(8, maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]));
    }
}
