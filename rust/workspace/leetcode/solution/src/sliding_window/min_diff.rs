// 1984. Minimum Difference Between Highest and Lowest of K Scores
pub fn min_diff(nums: &[i32], k: usize) -> i32 {
    let n = nums.len();
    if k == 1 {
        return 0;
    }
    let mut min = i32::MAX;
    for i in 0..n - 1 {
        for j in i + 1..n {
            min = min.min((nums[i] - nums[j]).abs());
        }
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
    }
}
