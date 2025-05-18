pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut profit = 0;

    let mut buy = prices[0] + fee;
    for i in 1..prices.len() {
        let price = prices[i] + fee;
        if price < buy {
            buy = price;
        } else if prices[i] > buy {
            profit += prices[i] - buy;
            buy = prices[i];
        }
    }

    profit
}

// pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
//     let mut min_price = prices[0];
//     let mut max_profit = 0;
//     let mut ans = 0;

//     for i in 1..prices.len() {
//         if min_price + max_profit > prices[i] + fee {
//             if max_profit > fee {
//                 ans += max_profit - fee;
//             }
//             min_price = prices[i];
//             max_profit = 0;
//         } else {
//             min_price = min_price.min(prices[i]);
//             max_profit = max_profit.max(prices[i] - min_price);
//         }
//     }

//     if max_profit > fee {
//         ans += max_profit - fee;
//     }

//     ans
// }
