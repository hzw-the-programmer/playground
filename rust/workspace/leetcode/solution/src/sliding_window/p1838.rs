// 1838. Frequency of the Most Frequent Element

pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = 0;
    let mut sum = 0;
    let mut l = 0;
    for r in 0..nums.len() {
        while l < r && sum + k < nums[r] * (r - l) as i32 {
            sum -= nums[l];
            l += 1;
        }
        sum += nums[r];
        res = res.max(r - l + 1);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_frequency(vec![1, 2, 4], 5), 3);
        assert_eq!(max_frequency(vec![1, 4, 8, 13], 5), 2);
        assert_eq!(max_frequency(vec![3, 9, 6], 2), 1);
    }
}
