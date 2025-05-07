// time: O(n^n)
// spave: O(n)
pub fn cut_rod(prices: &[i32], n: usize) -> i32 {
    if n == 0 {
        0
    } else {
        let mut ans = 0;
        for i in 1..(n + 1).min(prices.len() + 1) {
            ans = ans.max(prices[i - 1] + cut_rod(prices, n - i));
        }
        ans
    }
}

// pub fn cut_rod(prices: &[i32], n: usize) -> i32 {
//     if n == 0 {
//         0
//     } else {
//         let mut ans = 0;
//         for i in 1..n + 1 {
//             if i - 1 < prices.len() {
//                 ans = ans.max(prices[i - 1] + cut_rod(prices, n - i));
//             }
//         }
//         ans
//     }
// }

// pub fn cut_rod(prices: &[i32], n: usize) -> i32 {
//     if n == 0 {
//         0
//     } else {
//         let mut ans = 0;
//         for i in 1..n + 1 {
//             for j in 1..i + 1 {
//                 if j - 1 < prices.len() {
//                     ans = ans.max(prices[j - 1] + cut_rod(prices, i - j));
//                 }
//             }
//         }
//         ans
//     }
// }
