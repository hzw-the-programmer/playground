// 643. Maximum Average Subarray
pub fn max_min_sum<F>(nums: &[i32], k: usize, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let mut sum: i32 = nums[..k].iter().sum();

    let mut res = sum;
    for i in k..nums.len() {
        sum += nums[i] - nums[i - k];
        res = f(res, sum);
    }

    res
}

pub fn max_average(nums: &[i32], k: usize) -> f32 {
    max_min_sum(nums, k, std::cmp::max) as f32 / k as f32
}

// cargo test max_average -p solution
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12.75, max_average(&[1, 12, -5, -6, 50, 3], 4));
        assert_eq!(5., max_average(&[5], 1));
    }
}
