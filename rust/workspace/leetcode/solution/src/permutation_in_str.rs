pub fn check_inclusion(s1: &str, s2: &str) -> bool {
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

    if cnt1 == cnt2 {
        return true;
    }

    for i in n1..n2 {
        cnt2[(s2[i] - b'a') as usize] += 1;
        cnt2[(s2[i - n1] - b'a') as usize] -= 1;

        if cnt1 == cnt2 {
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
        assert_eq!(true, check_inclusion("ab", "eidbaooo"));
        assert_eq!(false, check_inclusion("ab", "eidboaoo"));
    }
}
