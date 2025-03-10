// 3. Longest Substring Without Repeating Characters

use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut res = 0;
    let (mut l, mut r) = (0, 0);
    let mut set = HashSet::new();
    set.insert(s[0]);
    while l < n {
        if l > 0 {
            set.remove(&s[l-1]);
        }
        while r < n-1 && !set.contains(&s[r+1]) {
            r += 1;
            set.insert(s[r]);
        }
        res = res.max(r-l+1);
        l += 1;
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