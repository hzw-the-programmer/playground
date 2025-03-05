// 643. Maximum Average Subarray
use super::max_min_sum::max_min_sum;

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
