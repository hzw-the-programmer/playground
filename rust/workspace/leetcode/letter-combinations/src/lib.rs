#![allow(dead_code)]

fn letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let phone_map = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let mut result = vec![];
    let mut current = String::new();
    backtrack(digits, &phone_map, 0, &mut current, &mut result);
    result
}

fn backtrack(
    digits: &str,
    phone_map: &[&str],
    index: usize,
    current: &mut String,
    result: &mut Vec<String>,
) {
    if index == digits.len() {
        result.push(current.clone());
        return;
    }

    let digit = digits.chars().nth(index).unwrap().to_digit(10).unwrap() as usize;
    let letters = phone_map[digit];
    for letter in letters.chars() {
        current.push(letter);
        backtrack(digits, phone_map, index + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let wanted = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let got = letter_combinations("23");
        assert_eq!(&wanted[..], &got[..]);
    }

    #[test]
    fn test_2() {
        let wanted: [&str; 0] = [];
        let got = letter_combinations("");
        assert_eq!(&wanted[..], &got[..]);
    }

    #[test]
    fn test_3() {
        let wanted = ["a", "b", "c"];
        let got = letter_combinations("2");
        assert_eq!(&wanted[..], &got[..]);
    }
}
