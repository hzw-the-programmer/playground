// 1456. Maximum Number of Vowels in a Substring of Given Length

use crate::Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let k = k as usize;

        let is_vowels =
            |i| s[i] == b'a' || s[i] == b'e' || s[i] == b'i' || s[i] == b'o' || s[i] == b'u';
        let mut max = 0;
        let mut ans = 0;

        for i in 0..n {
            if is_vowels(i) {
                max += 1;
            }

            if i >= k - 1 {
                if i > k - 1 && is_vowels(i - k) {
                    max -= 1;
                }
                ans = max.max(ans);
            }
        }

        ans as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_vowels() {
        assert_eq!(3, Solution::max_vowels("abciiidef".to_string(), 3));
        assert_eq!(2, Solution::max_vowels("aeiou".to_string(), 2));
        assert_eq!(2, Solution::max_vowels("leetcode".to_string(), 3));
        assert_eq!(0, Solution::max_vowels("rhythms".to_string(), 4));
        assert_eq!(1, Solution::max_vowels("tryhard".to_string(), 4));
    }
}
