// 8. String to Integer (atoi)

use crate::Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut state = State::Start;
        let mut ans = 0;
        let mut sign = 1;

        for c in s.chars() {
            state = state.next(c);
            match state {
                State::Start => {}
                State::Sign(s) => sign = s as i64,
                State::Digit(d) => {
                    ans = ans * 10 + d as i64;
                    if sign == 1 && ans > i32::MAX as i64 {
                        return i32::MAX;
                    } else if sign == -1 && -ans < i32::MIN as i64 {
                        return i32::MIN;
                    }
                }
                State::End => break,
            }
        }

        (ans * sign) as _
    }
}

enum State {
    Start,
    Sign(i8),
    Digit(u8),
    End,
}

impl State {
    fn next(&self, c: char) -> Self {
        match self {
            Self::Start => {
                if c.is_whitespace() {
                    Self::Start
                } else if c == '+' {
                    Self::Sign(1)
                } else if c == '-' {
                    Self::Sign(-1)
                } else if c.is_ascii_digit() {
                    Self::Digit(c.to_digit(10).unwrap() as u8)
                } else {
                    Self::End
                }
            }
            Self::Sign(_) => {
                if c.is_ascii_digit() {
                    Self::Digit(c.to_digit(10).unwrap() as u8)
                } else {
                    Self::End
                }
            }
            Self::Digit(_) => {
                if c.is_ascii_digit() {
                    Self::Digit(c.to_digit(10).unwrap() as u8)
                } else {
                    Self::End
                }
            }
            Self::End => Self::End,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(42, Solution::my_atoi("42".to_string()));
        assert_eq!(-42, Solution::my_atoi(" -042".to_string()));
        assert_eq!(1337, Solution::my_atoi("1337c0d3".to_string()));
        assert_eq!(0, Solution::my_atoi("0-1".to_string()));
        assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
    }
}
