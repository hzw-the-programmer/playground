// time: O(n*sum)
// space: O(n*sum)
pub fn subset_sum(nums: &[u32], sum: u32) -> bool {
    let sum = sum as usize;
    let n = nums.len();
    let mut dp = vec![vec![false; sum + 1]; n + 1];
    for i in 0..n + 1 {
        for j in 0..sum + 1 {
            if j == 0 {
                dp[i][j] = true;
            } else if i == 0 {
                dp[i][j] = false;
            } else {
                if nums[i - 1] > j as u32 {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
                }
            }
        }
    }
    dp[n][sum]
}
