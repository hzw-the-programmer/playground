// 1100. Find K-Length Substrings With No Repeated Characters
// 1 <= s.length <= 104
// s consists of lowercase English letters.
// 1 <= k <= 10^4

pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let k = k as usize;

    if k > n {
        return 0;
    }

    let mut ans = 0;
    let mut map = vec![0; 26];
    let mut count = 0;
    for i in 0..n {
        let idx = (s[i] - b'a') as usize;
        map[idx] += 1;
        if map[idx] == 1 {
            count += 1;
        }

        if i < k - 1 {
            continue;
        }

        if count == k {
            ans += 1;
        }

        let idx = (s[i + 1 - k] - b'a') as usize;
        map[idx] -= 1;
        if map[idx] == 0 {
            count -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_k_len_substr_no_repeats() {
        assert_eq!(
            num_k_len_substr_no_repeats("havefunonleetcode".to_string(), 5),
            6
        );
        assert_eq!(num_k_len_substr_no_repeats("home".to_string(), 5), 0);
    }
}
