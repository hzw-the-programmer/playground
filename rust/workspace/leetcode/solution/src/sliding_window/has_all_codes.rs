use super::bits::Bits;

// 1461. Check If a String Contains All Binary Codes of Size K
pub fn has_all_codes(s: &str, k: usize) -> bool {
    let s = s.as_bytes();
    let mut bits = Bits::new(1 << k);

    let mut index = 0;
    for i in 0..k {
        index = (index << 1) | (s[i] - b'0') as usize;
    }
    bits.set(index);

    let mask = (1 << k) - 1;
    for i in k..s.len() {
        index = ((index << 1) | (s[i] - b'0') as usize) & mask;

        bits.set(index);

        if bits.all_set() {
            return true;
        }
    }

    bits.all_set()
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
