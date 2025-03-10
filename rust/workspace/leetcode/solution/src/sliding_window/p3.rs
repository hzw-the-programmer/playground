/*
  3. Longest Substring Without Repeating Characters

  Constraints:
  * s consists of English letters, digits, symbols and spaces.
*/
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut res = 0;
    let mut set = HashSet::new();

    let mut r = 0;
    for l in 0..n {
        if l > 0 {
            set.remove(&s[l - 1]);
        }
        while r < n && !set.contains(&s[r]) {
            set.insert(s[r]);
            r += 1;
        }
        res = res.max(r - l);
    }
    res as i32
}

pub fn length_of_longest_substring_1(s: String) -> i32 {
    let s = s.as_bytes();
    let mut res = 0;
    let mut l = 0;
    let mut set = HashSet::new();
    for (r, &c) in s.iter().enumerate() {
        while set.contains(&c) {
            set.remove(&s[l]);
            l += 1;
        }
        set.insert(c);
        res = res.max(r-l+1);
    }
    res.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));

        assert_eq!(3, length_of_longest_substring("abccbcbb".to_string()));

        assert_eq!(3, length_of_longest_substring_1("abcabcbb".to_string()));
        assert_eq!(1, length_of_longest_substring_1("bbbbb".to_string()));
        assert_eq!(3, length_of_longest_substring_1("pwwkew".to_string()));

        assert_eq!(3, length_of_longest_substring_1("abccbcbb".to_string()));
    }
}