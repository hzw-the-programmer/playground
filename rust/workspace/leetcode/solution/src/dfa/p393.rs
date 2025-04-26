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
