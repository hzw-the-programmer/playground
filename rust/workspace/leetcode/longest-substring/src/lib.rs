#![allow(dead_code)]

use std::collections::HashMap;

fn longest_substring_len(s: &str) -> usize {
    let mut indices = HashMap::new();
    let mut max_len = 0;
    let mut start = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(index) = indices.get(&c) {
            start = start.max(index + 1);
        }
        indices.insert(c, i);
        max_len = max_len.max(i - start + 1);
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abcabcbb";
        assert_eq!(3, longest_substring_len(s));
    }

    #[test]
    fn test_2() {
        let s = "bbbbb";
        assert_eq!(1, longest_substring_len(s));
    }

    #[test]
    fn test_3() {
        let s = "pwwkew";
        assert_eq!(3, longest_substring_len(s));
    }
}
