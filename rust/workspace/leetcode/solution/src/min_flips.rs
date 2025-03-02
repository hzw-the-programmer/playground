pub fn min_flips(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut count = 0;
    let mut wanted = if s[0] == b'1' { b'0' } else { b'1' };
    for &c in &s[1..] {
        if c != wanted {
            count += 1;
        }
        wanted = if wanted == b'1' { b'0' } else { b'1' };
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, min_flips("111000"));
        assert_eq!(0, min_flips("010"));
        assert_eq!(1, min_flips("1110"));
    }
}
