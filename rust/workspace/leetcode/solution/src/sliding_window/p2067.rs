// 2067. Number of Equal Count Substrings
pub fn equal_count_substrings(s: String, count: usize) -> i32 {
    let s = s.as_bytes();
    let mut res = 0;

    for x in 1..27 {
        let k = x * count;
        if k > s.len() {
            break;
        }
        
        let mut map = [0; 26];
        let mut y = 0;
        for (i, &b) in s.iter().enumerate() {
            let index = (b - b'a') as usize;
            map[index] += 1;
            if map[index] == count {
                y += 1;
            } else if map[index] == count + 1 {
                y -= 1;
            }
            
            if i >= k {
                let index = (s[i-k] - b'a') as usize;
                map[index] -= 1;
                if map[index] == count {
                    y += 1;
                } else if map[index] == count - 1 {
                    y -= 1;
                }
            }

            if y == x {
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
        assert_eq!(3, equal_count_substrings("aaabcbbcc".to_string(), 3));
        assert_eq!(0, equal_count_substrings("abcd".to_string(), 2));
        assert_eq!(0, equal_count_substrings("a".to_string(), 5));
    }
}