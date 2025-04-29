// 2841. Maximum Sum of Almost Unique Subarray

use std::collections::HashMap;

pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
    let m = m as usize;
    let k = k as usize;

    let mut map = HashMap::new();
    let mut sum = 0;

    for i in 0..k {
        sum += nums[i] as i64;
        *map.entry(nums[i]).or_insert(0) += 1;
    }

    let mut ans = if map.len() >= m { sum } else { 0 };

    for i in k..nums.len() {
        sum += (nums[i] - nums[i - k]) as i64;
        *map.entry(nums[i]).or_insert(0) += 1;
        let v = map.get_mut(&nums[i - k]).unwrap();
        *v -= 1;
        if *v == 0 {
            map.remove(&nums[i - k]);
        }
        if map.len() >= m {
            ans = ans.max(sum);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum() {
        assert_eq!(18, max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4));
        assert_eq!(23, max_sum(vec![5, 9, 9, 2, 4, 5, 4], 1, 3));
        assert_eq!(0, max_sum(vec![1, 2, 1, 2, 1, 2, 1], 3, 3));
    }
}
