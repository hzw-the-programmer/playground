// 1016. Binary String With Substrings Representing 1 To N
pub fn query_string_1(s: String, n: i32) -> bool {
    for i in 1..=n {
        if !s.contains(&format!("{i:b}")) {
            return false;
        }
    }
    true
}

use std::collections::HashSet;

pub fn query_string_2(s: String, n: i32) -> bool {
    let mut set = HashSet::new();
    let s = s.as_bytes();
    let sn = s.len();
    for (i, &b) in s.iter().enumerate() {
        let b = b - b'0';
        if b == 0 {
            continue;
        }
        let mut x = 0;
        for j in i..sn {
            let b = s[j] - b'0';
            x = (x << 1) + b as i32;
            if x > n {
                break;
            }
            set.insert(x);
        }
    }
    set.len() == n as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, query_string_1("0110".to_string(), 3));
        assert_eq!(false, query_string_1("0110".to_string(), 4));

        assert_eq!(true, query_string_2("0110".to_string(), 3));
        assert_eq!(false, query_string_2("0110".to_string(), 4));
    }
}
