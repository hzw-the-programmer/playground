#![allow(dead_code)]

const BASESTR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
];

fn num2str(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        num2str(num / base, base) + BASESTR[(num % base) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num2str_test() {
        assert_eq!("1100100", num2str(100, 2));
        assert_eq!("144", num2str(100, 8));
        assert_eq!("64", num2str(100, 16));
    }
}
