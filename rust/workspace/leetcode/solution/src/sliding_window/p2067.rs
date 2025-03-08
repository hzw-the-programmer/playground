// 2067. Number of Equal Count Substrings
pub fn equal_count_substrings(s: String, count: usize) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut res = 0;
    for i in 1..27 {
        let k = i * count;
        if k > n {
            break;
        }
        let mut map = [0; 26];
        for j in 0..n {
            map[(s[j] - b'a') as usize] += 1;
            if j > k - 2 {
                if map.iter().filter(|&&x| x==count).count() == i {
                    res += 1;
                }
                map[(s[j+1-k]-b'a') as usize] -= 1;
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
        assert_eq!(3, equal_count_substrings("aaabcbbcc".to_string(), 3));
        assert_eq!(0, equal_count_substrings("abcd".to_string(), 2));
        assert_eq!(0, equal_count_substrings("a".to_string(), 5));
    }
}