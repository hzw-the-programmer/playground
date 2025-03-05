// 438. Find All Anagrams in a String

// sliding window
pub fn find_anagrams_1(s: &str, p: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let sl = s.len();
    let p = p.as_bytes();
    let pl = p.len();
    let mut res = vec![];

    if sl < pl {
        return res;
    }

    let mut s_cnt = [0; 26];
    let mut p_cnt = [0; 26];
    for i in 0..pl {
        s_cnt[(s[i] - b'a') as usize] += 1;
        p_cnt[(p[i] - b'a') as usize] += 1;
    }
    if s_cnt == p_cnt {
        res.push(0);
    }

    for i in pl..sl {
        s_cnt[(s[i] - b'a') as usize] += 1;
        s_cnt[(s[i - pl] - b'a') as usize] -= 1;
        if s_cnt == p_cnt {
            res.push(i - pl + 1);
        }
    }
    res
}

// sliding window optimize
pub fn find_anagrams_2(s: &str, p: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let sl = s.len();
    let p = p.as_bytes();
    let pl = p.len();
    let mut res = vec![];

    if sl < pl {
        return res;
    }

    let mut cnt = [0; 26];
    for i in 0..pl {
        cnt[(s[i] - b'a') as usize] += 1;
        cnt[(p[i] - b'a') as usize] -= 1;
    }
    let mut differ = cnt.iter().filter(|&&x| x != 0).count();
    if differ == 0 {
        res.push(0);
    }

    for i in pl..sl {
        let index = (s[i] - b'a') as usize;
        if cnt[index] == 0 {
            differ += 1;
        } else if cnt[index] == -1 {
            differ -= 1;
        }
        cnt[index] += 1;

        let index = (s[i - pl] - b'a') as usize;
        if cnt[index] == 0 {
            differ += 1;
        } else if cnt[index] == 1 {
            differ -= 1;
        }
        cnt[index] -= 1;

        if differ == 0 {
            res.push(i - pl + 1);
        }
    }

    res
}

// two pointers
pub fn find_anagrams_3(s: &str, p: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let sl = s.len();
    let p = p.as_bytes();
    let pl = p.len();
    let mut res = vec![];

    if sl < pl {
        return res;
    }

    let mut cnt = [0; 26];
    for &c in p.iter() {
        cnt[(c - b'a') as usize] -= 1;
    }

    let mut left = 0;
    for right in 0..sl {
        let i = (s[right] - b'a') as usize;
        cnt[i] += 1;
        while cnt[i] > 0 {
            cnt[(s[left] - b'a') as usize] -= 1;
            left += 1;
        }

        if right + 1 - left == pl {
            res.push(left);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(&[0, 6][..], find_anagrams_1("cbaebabacd", "abc"));
        assert_eq!(&[0, 1, 2][..], find_anagrams_1("abab", "ab"));

        assert_eq!(&[0, 6][..], find_anagrams_2("cbaebabacd", "abc"));
        assert_eq!(&[0, 1, 2][..], find_anagrams_2("abab", "ab"));

        assert_eq!(&[0, 6][..], find_anagrams_3("cbaebabacd", "abc"));
        assert_eq!(&[0, 1, 2][..], find_anagrams_3("abab", "ab"));
    }
}
