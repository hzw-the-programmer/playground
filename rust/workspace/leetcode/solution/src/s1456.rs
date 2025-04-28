// 1456. Maximum Number of Vowels in a Substring of Given Length

use crate::Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;

        let is_vowels = |b| b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u';
        let mut max = 0;
        let mut ans = 0;

        for (i, &b) in s.iter().enumerate() {
            // 1. enter window
            if is_vowels(b) {
                max += 1;
            }

            // 2. less than window size
            if i < k - 1 {
                continue;
            }

            // 3. update
            ans = max.max(ans);

            // 4. leave window
            if is_vowels(s[i + 1 - k]) {
                max -= 1;
            }
        }

        ans
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
