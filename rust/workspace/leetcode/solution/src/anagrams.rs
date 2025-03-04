pub fn find_anagrams(s: &str, p: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n1 = s.len();
    let p = p.as_bytes();
    let n2 = p.len();
    let mut res = vec![];
    if n1 < n2 {
        return res;
    }

    let mut cnt1 = [0; 26];
    let mut cnt2 = [0; 26];
    for i in 0..n2 {
        cnt1[(s[i] - b'a') as usize] += 1;
        cnt2[(p[i] - b'a') as usize] += 1;
    }
    if cnt1 == cnt2 {
        res.push(0);
    }

    for i in n2..n1 {
        cnt1[(s[i] - b'a') as usize] += 1;
        cnt1[(s[i - n2] - b'a') as usize] -= 1;
        if cnt1 == cnt2 {
            res.push(i - n2 + 1);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(&[0, 6][..], find_anagrams("cbaebabacd", "abc"));
        assert_eq!(&[0, 1, 2][..], find_anagrams("abab", "ab"));
    }
}
