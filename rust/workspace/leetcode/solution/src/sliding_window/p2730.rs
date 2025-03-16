// 2730. Find the Longest Semi-Repetitive Substring

pub fn longest_semi_repetitive_substring(s: String) -> i32 {
    let s = s.as_bytes();
    if s.len() == 1 {
        return 1;
    }
    let mut res = 0;
    let mut count = 0;
    let mut l = 0;
    for r in 1..s.len() {
        if s[r] == s[r - 1] {
            count += 1;
        }

        while count > 1 {
            l += 1;
            if s[l] == s[l - 1] {
                count -= 1;
            }
        }

        res = res.max(r - l + 1);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_semi_repetitive_substring("52233".to_string()), 4);
        assert_eq!(longest_semi_repetitive_substring("5494".to_string()), 4);
        assert_eq!(longest_semi_repetitive_substring("1111111".to_string()), 2);
        assert_eq!(longest_semi_repetitive_substring("0".to_string()), 1);
    }
}
