// 2461. Maximum Sum of Distinct Subarrays With Length K

// 1 <= nums[i] <= 100000
pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;

    let mut sum = 0;
    let mut map = vec![0; 100001];
    let mut count = 0;
    for i in 0..k {
        sum += nums[i] as i64;
        map[nums[i] as usize] += 1;
        if map[nums[i] as usize] == 1 {
            count += 1;
        }
    }
    let mut ans = if count == k { sum } else { 0 };

    for i in k..nums.len() {
        sum += (nums[i] - nums[i - k]) as i64;
        map[nums[i] as usize] += 1;
        if map[nums[i] as usize] == 1 {
            count += 1;
        }

        map[nums[i - k] as usize] -= 1;
        if map[nums[i - k] as usize] == 0 {
            count -= 1;
        }

        if count == k {
            ans = ans.max(sum);
        }
    }

    ans
}

pub fn maximum_subarray_sum_v2(nums: Vec<i32>, k: i32) -> i64 {
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

    fn test(f: fn(Vec<i32>, i32) -> i64) {
        assert_eq!(f(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
        assert_eq!(f(vec![4, 4, 4], 3), 0);
    }

    #[test]
    fn test_maximum_subarray_sum() {
        test(maximum_subarray_sum);
    }

    #[test]
    fn test_maximum_subarray_sum_v2() {
        test(maximum_subarray_sum_v2);
    }
}
