pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = i32::MAX;
    let mut max = 0;
    let mut ans = 0;
    for i in 0..prices.len() {
        if i > 0 && prices[i] < prices[i - 1] {
            ans += max;
            max = 0;
            buy = prices[i];
        } else {
            buy = buy.min(prices[i]);
            max = max.max(prices[i] - buy);
        }
    }
    ans + max
}
