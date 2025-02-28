/*
   byte 1: 0000 0000
   index : 7654 3210
   byte 2: 0000 0000
   index : fedc ba98
*/
pub struct Bits {
    bytes: Vec<u8>,
    len: usize,
}

impl Bits {
    pub fn new(len: usize) -> Bits {
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

    pub fn set(&mut self, index: usize) {
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

    pub fn all_set(&self) -> bool {
        self.bytes.iter().all(|b| b & 0xFF == 0xFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
