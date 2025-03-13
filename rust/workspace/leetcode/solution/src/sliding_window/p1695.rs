// 1695. Maximum Erasure Value

use std::collections::HashMap;

pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut sum = 0;
    let mut res = 0;
    let mut l = 0;
    for r in 0..nums.len() {
        *map.entry(nums[r]).or_insert(0) += 1;
        while *map.get(&nums[r]).unwrap() > 1 {
            let c = map.get_mut(&nums[l]).unwrap();
            *c -= 1;
            sum -= nums[l];
            l += 1;
        }
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
