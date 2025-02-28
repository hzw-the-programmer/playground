// 1461. Check If a String Contains All Binary Codes of Size K
pub fn has_all_codes(s: &str, k: usize) -> bool {
    let s = s.as_bytes();
    let mut bits = Bits::new(1 << k);

    let mut index = 0;
    for i in 0..k {
        index += (s[i] - b'0') as usize * (1 << i);
    }
    bits.set(index);

    for i in k..s.len() {
        index >>= 1;
        index += (s[i] - b'0') as usize * (1 << (k - 1));

        bits.set(index);
    }

    bits.all_set()
}

/*
   byte 1: 0000 0000
   index : 7654 3210
   byte 2: 0000 0000
   index : fedc ba98
*/
struct Bits {
    bytes: Vec<u8>,
    len: usize,
}

impl Bits {
    fn new(len: usize) -> Bits {
        assert!(len > 0);

        let mut b = Bits {
            bytes: vec![0; (len + 7) / 8],
            len,
        };

        if len % 8 != 0 {
            b.bytes[(len - 1) / 8] = !((1 << (len % 8)) - 1);
        }

        b
    }

    fn set(&mut self, index: usize) {
        assert!(index < self.len);
        self.bytes[index / 8] |= 1 << (index % 8);
    }

    fn clear(&mut self, index: usize) {
        #[cfg(not(test))]
        assert!(index < self.len);
        self.bytes[index / 8] &= !(1 << (index % 8));
    }

    fn is_set(&self, index: usize) -> bool {
        #[cfg(not(test))]
        assert!(index < self.len);
        self.bytes[index / 8] & (1 << (index % 8)) != 0
    }

    fn all_set(&self) -> bool {
        self.bytes.iter().all(|b| b & 0xFF == 0xFF)
    }
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

    #[test]
    fn bits() {
        /*
           byte 1: 0000 0000
           index : 7654 3210
           byte 2: 1110 0000
           index : fedc ba98
        */
        let len = 13;
        let max = ((len + 7) / 8) * 8;

        let mut bits = Bits::new(len);

        for i in 0..len {
            assert!(!bits.is_set(i));
        }
        for i in len..max {
            assert!(bits.is_set(i));
        }

        for i in 0..len {
            bits.set(i);
        }
        assert!(bits.all_set());

        for i in 0..max {
            bits.clear(i);
        }
        for i in 0..max {
            assert!(!bits.is_set(i));
        }
    }
}
