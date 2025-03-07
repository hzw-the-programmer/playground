// 2953. Count Complete Substrings
pub fn count_complete_substrings(word: String, k: i32) -> i32 {
    let word = word.as_bytes();
    let k = k as usize;
    let n = word.len();
    let mut res = 0;
    let (mut start, mut i) = (0, 1);
    while i < n {
        if (word[i] as i32 - word[i - 1] as i32).abs() > 2 {
            res += find(&word[start..i], k);
            start = i;
        }
        i += 1;
    }
    res += find(&word[start..i], k);
    res
}

fn find(s: &[u8], k: usize) -> i32 {
    let n = s.len();
    let mut res = 0;
    for i in (1..26usize).rev() {
        if i * k > n {
            continue;
        }
        let mut count = [0; 26];
        for j in 0..i * k {
            count[(s[j] - b'a') as usize] += 1;
        }
        if count.iter().filter(|&&x| x == k).count() == i {
            res += 1;
        }
        for j in i * k..n {
            count[(s[j] - b'a') as usize] += 1;
            count[(s[j - i * k] - b'a') as usize] -= 1;
            if count.iter().filter(|&&x| x == k).count() == i {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, count_complete_substrings("igigee".to_string(), 2));
        assert_eq!(6, count_complete_substrings("aaabbbccc".to_string(), 3));
    }
}
