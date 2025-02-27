use std::collections::HashMap;

// 1100. Find K-Length Substrings With No Repeated Characters
pub fn substrs_no_repeats(s: &[u8], k: usize) -> i32 {
    let n = s.len();
    if n < k {
        return 0;
    }

    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(s[i]).or_insert(0) += 1;
    }

    let mut count = 0;
    if map.len() == k {
        count += 1;
    }

    for i in k..n {
        *map.entry(s[i]).or_insert(0) += 1;

        let c = map.get_mut(&s[i - k]).unwrap();
        *c -= 1;
        if *c == 0 {
            map.remove(&s[i - k]);
        }

        if map.len() == k {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, substrs_no_repeats(b"havefunonleetcode", 5));
        assert_eq!(0, substrs_no_repeats(b"home", 5));
    }
}
