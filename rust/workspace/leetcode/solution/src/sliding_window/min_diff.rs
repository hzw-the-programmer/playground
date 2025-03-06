// 1984. Minimum Difference Between Highest and Lowest of K Scores
pub fn min_diff(nums: &[i32], k: usize) -> i32 {
    if k == 1 {
        return 0;
    }
    let mut nums = Vec::from(nums);
    nums.sort();
    let mut min = nums[k - 1] - nums[0];
    for i in k..nums.len() {
        min = min.min(nums[i] - nums[i - k + 1]);
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, min_diff(&[90], 1));
        assert_eq!(2, min_diff(&[9, 4, 1, 7], 2));
        assert_eq!(5, min_diff(&[1, 4, 7, 9], 3));
    }
}
