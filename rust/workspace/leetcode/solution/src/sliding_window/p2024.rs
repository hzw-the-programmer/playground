// 2024. Maximize the Confusion of an Exam

/*
  T 84 0x54 1001 1000
  F 70 0x46 0100 0110
  分别统计 T,F 的次数，保证 T,F 的次数不能同时 >k
  如果一个 >k，另一个 <=k，那么操作 <= k 的字符
*/
pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let s = answer_key.as_bytes();
    let mut res = 0;
    let mut cnt = [0; 2];
    let mut l = 0;
    for (r, &b) in s.iter().enumerate() {
        cnt[(b >> 1 & 1) as usize] += 1;
        while cnt[0] > k && cnt[1] > k {
            cnt[(s[l] >> 1 & 1) as usize] -= 1;
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
