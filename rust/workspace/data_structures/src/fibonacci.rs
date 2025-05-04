// https://cp-algorithms.com/dynamic_programming/intro-to-dp.html

// time: O(2^n)
// space: O(n)
pub fn fibonacci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// time: O(n)
// space: O(n)
pub fn fibonacci_memo(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    fibonacci_memo_recursive(n, &mut memo)
}

fn fibonacci_memo_recursive(n: usize, memo: &mut [usize]) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        if memo[n] == 0 {
            memo[n] = fibonacci(n - 1) + fibonacci(n - 2);
        }
        memo[n]
    }
}

// time: O(n)
// space: O(n)
pub fn fibonacci_dp(n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

// time: O(n)
// space: O(1)
pub fn fibonacci_dp_v2(n: usize) -> usize {
    let mut dp = [0, 1];
    for _ in 2..=n {
        let r = dp[0] + dp[1];
        dp[0] = dp[1];
        dp[1] = r;
    }
    dp[1]
}

// time: O(n)
// space: O(1)
pub fn fibonacci_dp_v3(n: usize) -> usize {
    let mut dp = [0, 1];
    for i in 2..=n {
        dp[i % 2] = dp[(i - 2) % 2] + dp[(i - 1) % 2];
    }
    dp[n % 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci_memo(10), 55);
        assert_eq!(fibonacci_dp(10), 55);
        assert_eq!(fibonacci_dp_v2(10), 55);
        assert_eq!(fibonacci_dp_v3(10), 55);
    }
}
