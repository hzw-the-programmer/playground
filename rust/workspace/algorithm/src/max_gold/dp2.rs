// time: O(n*m)
// space: O(n)
pub fn max_gold<const N: usize, const M: usize>(mat: &[[u32; M]; N]) -> u32 {
    let mut dp = [0; N];
    for i in 0..N {
        dp[i] = mat[i][M - 1];
    }

    for j in (0..M - 1).rev() {
        let mut diagonal = 0;
        for i in 0..N {
            let t = dp[i];
            dp[i] = 0;
            if i > 0 {
                dp[i] = dp[i].max(diagonal);
            }
            dp[i] = dp[i].max(t);
            if i + 1 < N {
                dp[i] = dp[i].max(dp[i + 1]);
            }
            dp[i] += mat[i][j];
            diagonal = t;
        }
    }

    let mut ans = 0;
    for i in 0..N {
        ans = ans.max(dp[i]);
    }
    ans
}
