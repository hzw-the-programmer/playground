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
