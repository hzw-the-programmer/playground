// https://www.geeksforgeeks.org/count-ways-reach-nth-stair/

// Recursion
// time: O(2^n)
// space: O(n)
pub fn count_ways(n: usize) -> usize {
    // base cases: if there are 0 or 1 stairs,
    // there is only one way to reach the top.
    if n == 0 || n == 1 {
        1
    } else {
        count_ways(n - 1) + count_ways(n - 2)
    }
}

// Top-Down DP (Memoization)
// time: O(n)
// space: O(n)
pub fn count_ways_memo(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    count_ways_memo_recursive(n, &mut memo)
}

fn count_ways_memo_recursive(n: usize, memo: &mut [usize]) -> usize {
    if n == 0 || n == 1 {
        1
    } else {
        if memo[n] == 0 {
            memo[n] =
                count_ways_memo_recursive(n - 1, memo) + count_ways_memo_recursive(n - 2, memo);
        }
        memo[n]
    }
}

// Bottom-Up DP (Tabulation)
// time: O(n)
// space: O(n)
pub fn count_ways_dp(n: usize) -> usize {
    if n == 0 {
        return 1;
    };
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

// Space Optimized DP
// time: O(n)
// space: O(1)
pub fn count_ways_dp_v2(n: usize) -> usize {
    let mut dp = [1, 1];
    for _ in 2..=n {
        let r = dp[0] + dp[1];
        dp[0] = dp[1];
        dp[1] = r;
    }
    dp[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(f: fn(usize) -> usize) {
        assert_eq!(f(0), 1);
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(f(3), 3);
        assert_eq!(f(4), 5);
        assert_eq!(f(5), 8);
    }

    #[test]
    fn test_count_ways() {
        test(count_ways);
        test(count_ways_memo);
        test(count_ways_dp);
        test(count_ways_dp_v2);
    }
}
