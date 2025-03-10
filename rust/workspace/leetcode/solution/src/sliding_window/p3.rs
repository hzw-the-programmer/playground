// 3. Longest Substring Without Repeating Characters

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));

        assert_eq!(3, length_of_longest_substring("abccbcbb".to_string()));
    }
}