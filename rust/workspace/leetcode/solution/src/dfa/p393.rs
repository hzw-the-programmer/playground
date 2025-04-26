// 393. UTF-8 Validation

use crate::Solution;

use State::*;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut state = OneByte;

        for n in data {
            state = state.next(n as u8);
            if state == Invalid {
                return false;
            }
        }

        state == OneByte
    }

    pub fn valid_utf8_v2(data: Vec<i32>) -> bool {
        let mut i = 0;

        while i < data.len() {
            let first_byte = data[i] as u8;

            let cnt = if first_byte & 0x80 == 0x00 {
                1
            } else if first_byte & 0xe0 == 0xc0 {
                2
            } else if first_byte & 0xf0 == 0xe0 {
                3
            } else if first_byte & 0xf8 == 0xf0 {
                4
            } else {
                return false;
            };

            if i + cnt > data.len() {
                return false;
            }

            for j in 1..cnt {
                if data[i + j] as u8 & 0xc0 != 0x80 {
                    return false;
                }
            }

            i += cnt;
        }

        true
    }
}

#[derive(PartialEq)]
enum State {
    OneByte,
    TwoByte,
    ThreeByte,
    FourByte,
    Invalid,
}

impl State {
    fn next(&self, b: u8) -> Self {
        match self {
            OneByte => {
                if b & 0b1000_0000 == 0b0000_0000 {
                    OneByte
                } else if b & 0b1110_0000 == 0b1100_0000 {
                    TwoByte
                } else if b & 0b1111_0000 == 0b1110_0000 {
                    ThreeByte
                } else if b & 0b1111_1000 == 0b1111_0000 {
                    FourByte
                } else {
                    Invalid
                }
            }
            TwoByte => {
                if b & 0b1100_0000 == 0b1000_0000 {
                    OneByte
                } else {
                    Invalid
                }
            }
            ThreeByte => {
                if b & 0b1100_0000 == 0b1000_0000 {
                    TwoByte
                } else {
                    Invalid
                }
            }
            FourByte => {
                if b & 0b1100_0000 == 0b1000_0000 {
                    ThreeByte
                } else {
                    Invalid
                }
            }
            Invalid => Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_utf8() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }
}
