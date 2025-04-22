// 2009. Minimum Number of Operations to Make Array Continuous

struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        nums.sort_unstable();
        nums.dedup();
        let mut left = 0;
        let mut ans = 0;
        for (i, &x) in nums.iter().enumerate() {
            if nums[left] < x - n + 1 {
                left += 1;
            }
            ans = ans.max(i - left + 1);
        }
        n - ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        // 2 3 4 5
        assert_eq!(0, Solution::min_operations(vec![4, 2, 5, 3]));
        // 1 2 3 (4) 5 6
        assert_eq!(1, Solution::min_operations(vec![1, 2, 3, 5, 6]));
        // 1 (2..=9) 10 (11..=99) 100 (101..=999) 1000
        assert_eq!(3, Solution::min_operations(vec![1, 10, 100, 1000]));
    }

    #[test]
    fn test_min_operations_1() {
        // 3 (4 5) 6 7 8
        assert_eq!(1, Solution::min_operations(vec![3, 6, 7, 8]));
        // 2 3 (4 5) 6 7 8
        assert_eq!(2, Solution::min_operations(vec![2, 3, 6, 7, 8]));
        // 1 2 3 (4 5) 6 7 8
        assert_eq!(2, Solution::min_operations(vec![1, 2, 3, 6, 7, 8]));
        // 1 2 (3) 4 5 (6) 7 8
        assert_eq!(2, Solution::min_operations(vec![1, 2, 4, 5, 7, 8]));

        // 0 1   2 3 4
        // 1 2 3 4 5 6
        assert_eq!(1, Solution::min_operations(vec![1, 2, 4, 5, 6]));

        // 1 2 3 (4) 5 6 7 8
        assert_eq!(1, Solution::min_operations(vec![1, 2, 3, 5, 6, 7, 8]));
        // 1 2 3 (4 5) 6 7 8 9
        assert_eq!(2, Solution::min_operations(vec![1, 2, 3, 6, 7, 8, 9]));
        // 1 2 3 (4 5 6) 7 8 9 10
        assert_eq!(3, Solution::min_operations(vec![1, 2, 3, 7, 8, 9, 10]));
        // 1 2 3 (4 5 6 7) 8 9 10 11
        assert_eq!(3, Solution::min_operations(vec![1, 2, 3, 8, 9, 10, 11]));

        // 1 2 (3 4 5) 6 7 8 (9 10 11 12) 13 14 15 16
        assert_eq!(
            4,
            Solution::min_operations(vec![1, 2, 6, 7, 8, 13, 14, 15, 16])
        );
        // 1 2 3 (4 5 6 7) 8 9 (10 11 12) 13 14 15 16
        assert_eq!(
            3,
            Solution::min_operations(vec![1, 2, 3, 8, 9, 13, 14, 15, 16])
        );

        // 1 (2) 3 (4) 5 (6) 7 (8) 9
        assert_eq!(2, Solution::min_operations(vec![1, 3, 5, 7, 9]));
    }
}
