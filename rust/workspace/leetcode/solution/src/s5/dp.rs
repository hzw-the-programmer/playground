pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    if n < 2 {
        return s;
    }

    let mut dp = vec![vec![false; n]; n];
    for i in 0..n {
        dp[i][i] = true;
    }

    let mut max_len = 0;
    let mut begin = 0;
    let bs = s.as_bytes();
    for len in 2..=n {
        for i in 0..n {
            let j = i + len - 1;
            if j >= n {
                break;
            }
            if bs[i] != bs[j] {
                dp[i][j] = false;
            } else {
                if len == 2 {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = dp[i + 1][j - 1];
                }
            }

            if dp[i][j] && j - i + 1 > max_len {
                max_len = j - i + 1;
                begin = i;
            }
        }
    }

    s[begin..begin + max_len].to_string()
}
