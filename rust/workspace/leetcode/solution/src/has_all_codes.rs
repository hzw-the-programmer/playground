// 1461. Check If a String Contains All Binary Codes of Size K
pub fn has_all_codes(s: &str, k: usize) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let total = 1 << k;
    let mut flags = vec![0u8; total];

    let mut index = 0;
    for i in 0..k {
        index += (s[i] - b'0') as usize * (1 << i);
    }
    flags[index] = 1;

    for i in k..n {
        index >>= 1;
        index += (s[i] - b'0') as usize * (1 << (k - 1));
        flags[index] = 1;
    }

    flags.iter().filter(|&&x| x == 1).count() == total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, has_all_codes("00110110", 2));
        assert_eq!(true, has_all_codes("0110", 1));
        assert_eq!(false, has_all_codes("0110", 2));
    }
}
