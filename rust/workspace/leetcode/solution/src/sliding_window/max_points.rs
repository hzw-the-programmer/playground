// 1423. Maximum Points You Can Obtain from Cards

use super::max_min_sum::max_min_sum;

pub fn max_points(nums: &[i32], k: usize) -> i32 {
    let total: i32 = nums.iter().sum();
    let min = max_min_sum(nums, nums.len() - k, std::cmp::min);
    total - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12, max_points(&[1, 2, 3, 4, 5, 6, 1], 3));
        assert_eq!(4, max_points(&[2, 2, 2], 2));
        assert_eq!(55, max_points(&[9, 7, 7, 9, 7, 7, 9], 7));
    }
}
