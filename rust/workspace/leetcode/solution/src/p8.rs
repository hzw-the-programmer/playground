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
                State::Sign(s) => sign = s,
                State::Number(n) => ans = ans * 10 + n as i32,
                State::End => break,
            }
        }
        ans * sign
    }
}

enum State {
    Start,
    Sign(i32),
    Number(u8),
    End,
}

impl State {
    fn next(&self, c: char) -> Self {
        match self {
            Self::Start => match c {
                ' ' => Self::Start,
                '-' => Self::Sign(-1),
                '+' => Self::Sign(1),
                '0'..='9' => Self::Number(c.to_digit(10).unwrap() as u8),
                _ => Self::End,
            },
            Self::Sign(_) => match c {
                '0'..='9' => Self::Number(c.to_digit(10).unwrap() as u8),
                _ => Self::End,
            },
            Self::Number(_) => match c {
                '0'..='9' => Self::Number(c.to_digit(10).unwrap() as u8),
                _ => Self::End,
            },
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
    }
}
