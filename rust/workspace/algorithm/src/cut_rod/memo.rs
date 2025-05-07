// time: O(n^2)
// space: O(n)
pub fn cut_rod(prices: &[i32], n: usize) -> i32 {
    let mut memo = vec![-1; n];
    cut_rod_recursive(prices, n, &mut memo)
}

fn cut_rod_recursive(prices: &[i32], n: usize, memo: &mut [i32]) -> i32 {
    if n == 0 {
        0
    } else {
        if memo[n - 1] == -1 {
            memo[n - 1] = 0;
            for i in 1..(n + 1).min(prices.len() + 1) {
                memo[n - 1] =
                    memo[n - 1].max(prices[i - 1] + cut_rod_recursive(prices, n - i, memo));
            }
        }
        memo[n - 1]
    }
}
