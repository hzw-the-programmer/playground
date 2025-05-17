pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut min_price = prices[0];
    let mut max_price = prices[0];
    let mut ans = 0;

    for i in 1..prices.len() {
        if max_price > prices[i] + fee {
            let profits = max_price - min_price;
            if profits > fee {
                ans += profits - fee;
            }
            min_price = prices[i];
            max_price = prices[i];
        } else {
            min_price = min_price.min(prices[i]);
            max_price = max_price.max(prices[i]);
        }
    }

    let profits = max_price - min_price;
    if profits > fee {
        ans += profits - fee;
    }

    ans
}
