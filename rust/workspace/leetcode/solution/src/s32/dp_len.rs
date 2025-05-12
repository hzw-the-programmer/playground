pub fn longest_valid_parentheses(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![0; n + 1];
    let mut ans = 0;
    for len in 2..=n {
        if s[len - 1] == b')' {
            if s[len - 2] == b'(' {
                dp[len] = dp[len - 2] + 2;
                ans = ans.max(dp[len]);
            } else if len - 2 >= dp[len - 1] && s[len - 2 - dp[len - 1]] == b'(' {
                dp[len] = dp[len - 1] + 2 + dp[len - 2 - dp[len - 1]];
                ans = ans.max(dp[len]);
            }
        }
    }
    ans as _
}
