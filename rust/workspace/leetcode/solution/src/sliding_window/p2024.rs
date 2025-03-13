// 2024. Maximize the Confusion of an Exam

pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let keys = answer_key.as_bytes();
    helper(keys, b'F', k).max(helper(keys, b'T', k))
}

fn helper(s: &[u8], b: u8, k: i32) -> i32 {
    let mut res = 0;
    let mut l = 0;
    let mut count = 0;
    for r in 0..s.len() {
        if s[r] == b {
            count += 1;
        }

        while count > k {
            if s[l] == b {
                count -= 1;
            }
            l += 1;
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
        assert_eq!(4, max_consecutive_answers("TTFF".to_string(), 2));
        assert_eq!(3, max_consecutive_answers("TFFT".to_string(), 1));
        assert_eq!(5, max_consecutive_answers("TTFTTFTT".to_string(), 1));
    }
}
