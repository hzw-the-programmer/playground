/*
  3090. Maximum Length Substring With Two Occurrences

  Constraints:
  * s consists only of lowercase English letters.
*/
pub fn maximum_length_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    let mut l = 0;
    let mut res = 0;
    for (r, &c) in s.iter().enumerate() {
        let index = (c - b'a') as usize;
        cnt[index] += 1;
        while cnt[index] > 2 {
            cnt[(s[l] - b'a') as usize] -= 1;
            l += 1;
        }
        res = res.max(r-l+1);
    }
    res.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, maximum_length_substring("bcbbbcba".to_string()));
        assert_eq!(2, maximum_length_substring("aaaa".to_string()));
    }
}