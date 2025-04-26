// 2781. Length of the Longest Valid Substring
// 1 <= forbidden[i].length <= 10

use crate::Solution;

impl Solution {
    // 执行用时分布 83ms 击败 100.00%
    // 消耗内存分布 9.75MB 击败 100.00%
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let word = word.as_bytes();
        // let forbidden = std::collections::HashSet::<Vec<u8>>::from_iter(
        //     forbidden.into_iter().map(|s| s.into_bytes()),
        // );
        // let forbidden =
        //     std::collections::HashSet::<&[u8]>::from_iter(forbidden.iter().map(|s| s.as_bytes()));
        let forbidden = forbidden
            .iter()
            .map(|s| s.as_bytes())
            .collect::<std::collections::HashSet<_>>();

        let mut ans = 0;
        let mut left = 0;
        for right in 0..word.len() {
            for i in (right.saturating_sub(9).max(left)..=right).rev() {
                if forbidden.contains(&word[i..=right]) {
                    left = i + 1;
                    break;
                }
            }
            ans = ans.max(right + 1 - left);
        }

        ans as _
    }

    // 执行用时分布 87ms 击败 100.00%
    // 消耗内存分布 9.68MB 击败 100.00%
    pub fn longest_valid_substring_v1(word: String, forbidden: Vec<String>) -> i32 {
        let word = word.as_bytes();
        // let forbidden = std::collections::HashSet::<Vec<u8>>::from_iter(
        //     forbidden.into_iter().map(|s| s.into_bytes()),
        // );
        // let forbidden =
        //     std::collections::HashSet::<&[u8]>::from_iter(forbidden.iter().map(|s| s.as_bytes()));
        let forbidden = forbidden
            .iter()
            .map(|s| s.as_bytes())
            .collect::<std::collections::HashSet<_>>();

        let mut ans = 0;
        let mut left = 0;
        for right in 0..word.len() {
            let mut i = right;
            while i >= left && right - i + 1 <= 10 {
                if forbidden.contains(&word[i..=right]) {
                    left = i + 1;
                    break;
                } else if i == 0 {
                    break;
                }
                i -= 1;
            }
            ans = ans.max(right + 1 - left);
        }

        ans as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            4,
            Solution::longest_valid_substring(
                "cbaaaabc".to_string(),
                ["aaa", "cb"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            )
        );
        assert_eq!(
            4,
            Solution::longest_valid_substring(
                "leetcode".to_string(),
                ["de", "le", "e"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            )
        );
        assert_eq!(
            100000,
            Solution::longest_valid_substring(
                std::iter::repeat('a').take(100000).collect::<String>(),
                ["bc", "de", "b"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            )
        );
    }
}
