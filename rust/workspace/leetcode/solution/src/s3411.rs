// 3411. Maximum Subarray With Equal Products

use crate::Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_length() {
        assert_eq!(5, Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]));
        assert_eq!(3, Solution::max_length(vec![2, 3, 4, 5, 6]));
        assert_eq!(5, Solution::max_length(vec![1, 2, 3, 1, 4, 5, 1]));
    }
}
