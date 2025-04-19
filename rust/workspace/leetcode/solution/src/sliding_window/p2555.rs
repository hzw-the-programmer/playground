// 2555. Maximize Win From Two Segments

struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        0
    }

    pub fn maximize_win_1(prize_positions: Vec<i32>, k: i32) -> i32 {
        let len = prize_positions.len();
        let mut right = 0;
        let mut res = 0;
        for (left, pos) in prize_positions.iter().enumerate() {
            while right < len && prize_positions[right] - prize_positions[left] <= k {
                right += 1;
            }
            if right == len {
                return res.max(right - left) as _;
            }
            res = res.max(right - left);
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_maximize_win() {
        assert_eq!(7, Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2));
        assert_eq!(2, Solution::maximize_win(vec![1, 2, 3, 4], 0));
    }

    #[test]
    fn test_maximize_win_1() {
        assert_eq!(6, Solution::maximize_win_1(vec![1, 1, 2, 2, 3, 3, 5], 2));
        assert_eq!(1, Solution::maximize_win_1(vec![1, 2, 3, 4], 0));
    }
}
