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
    for (i, b) in s.bytes().enumerate() {
        if b - b'0' == 0 {
            continue;
        }
        let mut x = 0;
        for b in s.bytes().skip(i) {
            let b = (b - b'0') as i32;
            x = (x << 1) + b;
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
