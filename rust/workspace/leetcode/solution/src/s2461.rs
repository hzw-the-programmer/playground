// 2461. Maximum Sum of Distinct Subarrays With Length K

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;

    let mut sum = 0;
    let mut map = std::collections::HashMap::new();
    for i in 0..k {
        sum += nums[i] as i64;
        *map.entry(nums[i]).or_insert(0) += 1;
    }
    let mut ans = if map.len() == k { sum } else { 0 };

    for i in k..nums.len() {
        sum += (nums[i] - nums[i - k]) as i64;
        *map.entry(nums[i]).or_insert(0) += 1;

        let v = map.get_mut(&nums[i - k]).unwrap();
        *v -= 1;
        if *v == 0 {
            map.remove(&nums[i - k]);
        }
        if map.len() == k {
            ans = ans.max(sum);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_subarray_sum() {
        assert_eq!(maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
        assert_eq!(maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
