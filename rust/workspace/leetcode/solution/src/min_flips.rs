pub fn min_flips(s: &str) -> i32 {
    let s = s.as_bytes();

    let mut start_0 = 0;
    let mut start_1 = 0;
    for (i, &c) in s.iter().enumerate() {
        if i % 2 == 0 {
            if c == b'1' {
                start_0 += 1;
            } else {
                start_1 += 1;
            }
        } else {
            if c == b'0' {
                start_0 += 1;
            } else {
                start_1 += 1;
            }
        }
    }

    if s.len() % 2 == 0 {
        return start_0.min(start_1);
    }

    let mut min = start_0.min(start_1);
    for &c in s.iter() {
        std::mem::swap(&mut start_0, &mut start_1);
        if c == b'0' {
            start_1 += 1;
        } else {
            start_0 += 1;
        }
        min = min.min(start_0).min(start_1);
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, min_flips("1101"));
        assert_eq!(0, min_flips("110"));

        assert_eq!(2, min_flips("111000"));
        assert_eq!(0, min_flips("010"));
        assert_eq!(1, min_flips("1110"));
    }
}
