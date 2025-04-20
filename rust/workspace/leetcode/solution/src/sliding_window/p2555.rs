// 2555. Maximize Win From Two Segments

struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let n = prize_positions.len();
        let mut dp = vec![0; n + 1];
        let mut ans = 0;
        for (right, &p) in prize_positions.iter().enumerate() {
            let left = prize_positions.partition_point(|&x| x < (p - k));
            let covered = right - left + 1;
            ans = ans.max(covered + dp[left]);
            dp[right + 1] = dp[right].max(covered);
            // println!(
            //     "({right}, {left}), covered={covered}, dp[{left}]={}, dp[{right}]={}",
            //     dp[left], dp[right]
            // );
            // println!("dp[{}]={}", right + 1, dp[right + 1]);
        }
        ans as _
    }

    pub fn maximize_win_1(prize_positions: Vec<i32>, k: i32) -> i32 {
        let len = prize_positions.len();
        let mut right = 0;
        let mut res = 0;
        for (left, &pos) in prize_positions.iter().enumerate() {
            while right < len && prize_positions[right] - pos <= k {
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

    #[test]
    fn test_maximize_win() {
        //   0 1 2   3 4 5 6
        // 0 1 2 3 4 5 6 7 8
        assert_eq!(6, Solution::maximize_win(vec![1, 2, 3, 5, 6, 7, 8], 2));

        // assert_eq!(7, Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2));
        // assert_eq!(2, Solution::maximize_win(vec![1, 2, 3, 4], 0));
    }

    // #[test]
    fn test_maximize_win_1() {
        assert_eq!(6, Solution::maximize_win_1(vec![1, 1, 2, 2, 3, 3, 5], 2));
        assert_eq!(1, Solution::maximize_win_1(vec![1, 2, 3, 4], 0));
    }
}
