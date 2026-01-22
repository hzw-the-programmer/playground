// 1297. Maximum Number of Occurrences of a Substring

pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_freq() {
        assert_eq!(max_freq("aababcaab".to_string(), 2, 3, 4), 2);
        assert_eq!(max_freq("aaaa".to_string(), 1, 3, 3), 2);
        assert_eq!(max_freq("aabcabcab".to_string(), 2, 2, 3), 3);
        assert_eq!(max_freq("abcde".to_string(), 2, 3, 3), 0);
    }
}
