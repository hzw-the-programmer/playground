// 65. Valid Number

use crate::Solution;

use State::*;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = Start;

        for c in s.chars() {
            state = state.next(c);
            if state == Invalid {
                return false;
            }
        }

        matches!(
            state,
            Integer | DotAfterInteger | Decimal | IntegerAfterE | End
        )
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    SignBeforeE,
    Integer,
    DotBeforeInteger,
    DotAfterInteger,
    Decimal,
    E,
    SignAfterE,
    IntegerAfterE,
    End,
    Invalid,
}

impl State {
    fn next(&self, c: char) -> Self {
        match self {
            Start => match c {
                ' ' => Start,
                '+' | '-' => SignBeforeE,
                '0'..='9' => Integer,
                '.' => DotBeforeInteger,
                _ => Invalid,
            },
            SignBeforeE => match c {
                '0'..='9' => Integer,
                '.' => DotBeforeInteger,
                _ => Invalid,
            },
            Integer => match c {
                '0'..='9' => Integer,
                '.' => DotAfterInteger,
                'e' | 'E' => E,
                ' ' => End,
                _ => Invalid,
            },
            DotBeforeInteger => match c {
                '0'..='9' => Decimal,
                _ => Invalid,
            },
            DotAfterInteger => match c {
                '0'..='9' => Decimal,
                'e' | 'E' => E,
                ' ' => End,
                _ => Invalid,
            },
            Decimal => match c {
                '0'..='9' => Decimal,
                'e' | 'E' => E,
                ' ' => End,
                _ => Invalid,
            },
            E => match c {
                '+' | '-' => SignAfterE,
                '0'..='9' => IntegerAfterE,
                _ => Invalid,
            },
            SignAfterE => match c {
                '0'..='9' => IntegerAfterE,
                _ => Self::Invalid,
            },
            IntegerAfterE => match c {
                '0'..='9' => IntegerAfterE,
                ' ' => End,
                _ => Invalid,
            },
            End => match c {
                ' ' => End,
                _ => Invalid,
            },
            Invalid => Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert!(Solution::is_number("2".to_string()));
        assert!(Solution::is_number("0089".to_string()));
        assert!(Solution::is_number("-0.1".to_string()));
        assert!(Solution::is_number("+3.14".to_string()));
        assert!(Solution::is_number("4.".to_string()));
        assert!(Solution::is_number("-.9".to_string()));
        assert!(Solution::is_number("2e10".to_string()));
        assert!(Solution::is_number("-90E3".to_string()));
        assert!(Solution::is_number("3e+7".to_string()));
        assert!(Solution::is_number("+6e-1".to_string()));
        assert!(Solution::is_number("53.5e93".to_string()));
        assert!(Solution::is_number("-123.456e789".to_string()));

        assert!(Solution::is_number("0".to_string()));
    }

    #[test]
    fn test_is_not_number() {
        assert!(!Solution::is_number("abc".to_string()));
        assert!(!Solution::is_number("1a".to_string()));
        assert!(!Solution::is_number("1e".to_string()));
        assert!(!Solution::is_number("e3".to_string()));
        assert!(!Solution::is_number("99e2.5".to_string()));
        assert!(!Solution::is_number("--6".to_string()));
        assert!(!Solution::is_number("-+3".to_string()));
        assert!(!Solution::is_number("95a54e53".to_string()));

        assert!(!Solution::is_number("e".to_string()));
        assert!(!Solution::is_number(".".to_string()));
    }
}
