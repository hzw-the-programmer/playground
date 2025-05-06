// time: O(n*m)
// space: O(n*m)
pub fn max_gold<const N: usize, const M: usize>(mat: &[[u32; M]; N]) -> u32 {
    let mut dp = [[0; M]; N];

    for i in 0..N {
        dp[i][M - 1] = mat[i][M - 1];
    }

    for j in (0..M - 1).rev() {
        for i in 0..N {
            dp[i][j] = 0;

            if i > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j + 1]);
            }
            
            dp[i][j] = dp[i][j].max(dp[i][j + 1]);
            
            if i + 1 < N {
                dp[i][j] = dp[i][j].max(dp[i + 1][j + 1]);
            }

            dp[i][j] += mat[i][j];
        }
    }

    let mut ans = 0;
    for i in 0..N {
        ans = ans.max(dp[i][0]);
    }
    ans
}
