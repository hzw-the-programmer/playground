// 1100. Find K-Length Substrings With No Repeated Characters
pub fn substrs_no_repeats(s: &str, k: usize) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    if n < k {
        return 0;
    }

    let mut map = [0; 26];
    let mut uniq = 0;
    for i in 0..k {
        let index = (s[i] - b'a') as usize;
        if map[index] == 0 {
            uniq += 1;
        }
        map[index] += 1;
    }

    let mut count = 0;
    if uniq == k {
        count += 1;
    }

    for i in k..n {
        let index = (s[i] - b'a') as usize;
        if map[index] == 0 {
            uniq += 1;
        }
        map[index] += 1;

        let index = (s[i - k] - b'a') as usize;
        map[index] -= 1;
        if map[index] == 0 {
            uniq -= 1;
        }

        if uniq == k {
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
        assert_eq!(6, substrs_no_repeats("havefunonleetcode", 5));
        assert_eq!(0, substrs_no_repeats("home", 5));
    }
}
