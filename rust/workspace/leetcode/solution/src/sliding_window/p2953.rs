// 2953. Count Complete Substrings

use super::p2067::helper;

pub fn count_complete_substrings(word: String, k: i32) -> i32 {
    let word = word.as_bytes();
    let k = k as usize;
    let n = word.len();
    let mut res = 0;
    let mut i = 0;
    while i < n {
        let start = i;
        i += 1;
        while i < n && (word[i] as i32 - word[i - 1] as i32).abs() <= 2 {
            i += 1;
        }
        res += helper(&word[start..i], k);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, count_complete_substrings("igigee".to_string(), 2));
        assert_eq!(6, count_complete_substrings("aaabbbccc".to_string(), 3));
    }
}
