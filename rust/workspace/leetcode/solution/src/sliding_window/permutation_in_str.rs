// 567. Permutaion in String

// sliding window
pub fn check_inclusion_1(s1: &str, s2: &str) -> bool {
    let s1 = s1.as_bytes();
    let n1 = s1.len();
    let s2 = s2.as_bytes();
    let n2 = s2.len();

    if n1 > n2 {
        return false;
    }

    let mut cnt1 = [0; 26];
    let mut cnt2 = [0; 26];
    for i in 0..n1 {
        cnt1[(s1[i] - b'a') as usize] += 1;
        cnt2[(s2[i] - b'a') as usize] += 1;
    }
    if cnt2 == cnt1 {
        return true;
    }

    for i in n1..n2 {
        cnt2[(s2[i] - b'a') as usize] += 1;
        cnt2[(s2[i - n1] - b'a') as usize] -= 1;
        if cnt2 == cnt1 {
            return true;
        }
    }

    false
}

// sliding window optimize
pub fn check_inclusion_2(s1: &str, s2: &str) -> bool {
    let s1 = s1.as_bytes();
    let n1 = s1.len();
    let s2 = s2.as_bytes();
    let n2 = s2.len();

    if n1 > n2 {
        return false;
    }

    let mut cnt = [0; 26];
    for i in 0..n1 {
        cnt[(s1[i] - b'a') as usize] -= 1;
        cnt[(s2[i] - b'a') as usize] += 1;
    }

    let mut differ = cnt.iter().filter(|&&x| x != 0).count();
    if differ == 0 {
        return true;
    }

    for i in n1..n2 {
        if cnt[(s2[i] - b'a') as usize] == 0 {
            differ += 1;
        }
        cnt[(s2[i] - b'a') as usize] += 1;
        if cnt[(s2[i] - b'a') as usize] == 0 {
            differ -= 1;
        }

        if cnt[(s2[i - n1] - b'a') as usize] == 0 {
            differ += 1;
        }
        cnt[(s2[i - n1] - b'a') as usize] -= 1;
        if cnt[(s2[i - n1] - b'a') as usize] == 0 {
            differ -= 1;
        }

        if differ == 0 {
            return true;
        }
    }

    false
}

// two pointers
pub fn check_inclusion_3(s1: &str, s2: &str) -> bool {
    let s1 = s1.as_bytes();
    let n1 = s1.len();
    let s2 = s2.as_bytes();
    let n2 = s2.len();

    if n1 > n2 {
        return false;
    }

    let mut cnt = [0; 26];
    for &c in s1.iter() {
        cnt[(c - b'a') as usize] -= 1;
    }

    let mut left = 0;
    for right in 0..n2 {
        let i = (s2[right] - b'a') as usize;
        cnt[i] += 1;
        while cnt[i] > 0 {
            cnt[(s2[left] - b'a') as usize] -= 1;
            left += 1;
        }
        if right + 1 - left == n1 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, check_inclusion_1("ab", "eidbaooo"));
        assert_eq!(false, check_inclusion_1("ab", "eidboaoo"));

        assert_eq!(true, check_inclusion_2("ab", "eidbaooo"));
        assert_eq!(false, check_inclusion_2("ab", "eidboaoo"));

        assert_eq!(true, check_inclusion_3("ab", "eidbaooo"));
        assert_eq!(false, check_inclusion_3("ab", "eidboaoo"));
    }
}
