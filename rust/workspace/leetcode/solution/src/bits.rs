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
    const ELEM_SIZE: usize = 8;
    const MASK: u8 = 0xFF;

    pub fn new(len: usize) -> Bits {
        assert!(len > 0);

        let mut b = Bits {
            bytes: vec![0; (len + (Self::ELEM_SIZE - 1)) / Self::ELEM_SIZE],
            len,
        };

        if len % Self::ELEM_SIZE != 0 {
            b.bytes[(len - 1) / Self::ELEM_SIZE] = !((1 << (len % Self::ELEM_SIZE)) - 1);
        }

        b
    }

    fn len(&self) -> usize {
        self.len
    }

    fn cap(&self) -> usize {
        self.bytes.len() * Self::ELEM_SIZE
    }

    pub fn set(&mut self, index: usize) {
        assert!(index < self.len);
        self.bytes[index / Self::ELEM_SIZE] |= 1 << (index % Self::ELEM_SIZE);
    }

    fn clear(&mut self, index: usize) {
        #[cfg(not(test))]
        assert!(index < self.len);
        self.bytes[index / Self::ELEM_SIZE] &= !(1 << (index % Self::ELEM_SIZE));
    }

    fn is_set(&self, index: usize) -> bool {
        #[cfg(not(test))]
        assert!(index < self.len);
        self.bytes[index / Self::ELEM_SIZE] & (1 << (index % Self::ELEM_SIZE)) != 0
    }

    pub fn all_set(&self) -> bool {
        self.bytes.iter().all(|b| b & Self::MASK == Self::MASK)
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
        let mut bits = Bits::new(13);

        for i in 0..bits.len() {
            assert!(!bits.is_set(i));
        }
        for i in bits.len()..bits.cap() {
            assert!(bits.is_set(i));
        }

        for i in 0..bits.len() {
            bits.set(i);
        }
        assert!(bits.all_set());

        for i in 0..bits.cap() {
            bits.clear(i);
        }
        for i in 0..bits.cap() {
            assert!(!bits.is_set(i));
        }
    }
}
