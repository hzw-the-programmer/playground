// 643. Maximum Average Subarray I

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;

    let mut max = nums[0..k].iter().sum::<i32>();
    let mut ans = max;

    for i in k..nums.len() {
        max += nums[i];
        max -= nums[i - k];
        ans = ans.max(max);
    }

    ans as f64 / k as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_average() {
        assert_eq!(12.75, find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
        assert_eq!(5.00000, find_max_average(vec![5], 1));
    }
}
