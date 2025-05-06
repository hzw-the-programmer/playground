// time: O(n*sum)
// space: O(sum)
pub fn subset_sum(nums: &[u32], sum: u32) -> bool {
    let n = nums.len();
    let sum = sum as usize;
    let mut dp = vec![false; sum + 1];
    dp[0] = true;

    for i in 1..n + 1 {
        for j in (1..sum + 1).rev() {
            if nums[i - 1] <= j as u32 {
                dp[j] = dp[j] || dp[j - nums[i - 1] as usize];
            }
        }
    }

    dp[sum]
}

// pub fn subset_sum(nums: &[u32], sum: u32) -> bool {
//     let n = nums.len();
//     let sum = sum as usize;
//     let mut pre = vec![false; sum + 1];
//     pre[0] = true;
//     let mut cur = vec![false; sum + 1];

//     for i in 1..n + 1 {
//         for j in 1..sum + 1 {
//             if nums[i - 1] > j as u32 {
//                 cur[j] = pre[j];
//             } else {
//                 cur[j] = pre[j] || pre[j - nums[i - 1] as usize];
//             }
//         }
//         std::mem::swap(&mut pre, &mut cur);
//     }

//     cur[sum]
// }
