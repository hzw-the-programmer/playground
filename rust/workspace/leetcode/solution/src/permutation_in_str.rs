pub fn check_inclusion(s1: &str, s2: &str) -> bool {
    let s1 = s1.as_bytes();
    let n1 = s1.len();
    let s2 = s2.as_bytes();
    let n2 = s2.len();

    if n1 > n2 {
        return false;
    }

    let mut cnt = [0; 26];
    for i in 0..n1 {
        cnt[(s2[i] - b'a') as usize] += 1;
        cnt[(s1[i] - b'a') as usize] -= 1;
    }

    let mut differ = cnt.iter().filter(|&&x| x != 0).count();
    if differ == 0 {
        return true;
    }

    for i in n1..n2 {
        if cnt[(s2[i] - b'a') as usize] == 0 {
            differ += 1;
        } else if cnt[(s2[i] - b'a') as usize] == -1 {
            differ -= 1;
        }
        cnt[(s2[i] - b'a') as usize] += 1;

        if cnt[(s2[i - n1] - b'a') as usize] == 0 {
            differ += 1;
        } else if cnt[(s2[i - n1] - b'a') as usize] == 1 {
            differ -= 1;
        }
        cnt[(s2[i - n1] - b'a') as usize] -= 1;

        if differ == 0 {
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
