// 1208. Get Equal Substrings Within Budget

/*
  abcd  abcd  abcd  abcd
  bcdf  cdef  acde  bcde
  1112  2222  0111  1111
*/
pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let n = s.len();
    let mut costs = vec![0; n];
    for i in 0..n {
        costs[i] = (s[i] as i32 - t[i] as i32).abs();
    }

    let mut l = 0;
    let mut res = 0;
    let mut sum = 0;
    for r in 0..n {
        sum += costs[r] as i32;

        while sum > max_cost {
            sum -= costs[l] as i32;
            l += 1;
        }

        let len = if r >= l { r - l + 1 } else { 0 };
        res = res.max(len);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            3,
            equal_substring("abcd".to_string(), "bcdf".to_string(), 3)
        );
        assert_eq!(
            1,
            equal_substring("abcd".to_string(), "cdef".to_string(), 3)
        );
        assert_eq!(
            1,
            equal_substring("abcd".to_string(), "acde".to_string(), 0)
        );
        assert_eq!(
            0,
            equal_substring("abcd".to_string(), "bcde".to_string(), 0)
        );
    }
}
