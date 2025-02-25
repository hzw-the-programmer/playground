pub fn minimum_recolors(s: &[u8], k: usize) -> usize {
    if s.len() < k {
        return 0;
    }

    let mut count = 0;
    for i in 0..k {
        if s[i] == b'W' {
            count += 1;
        }
    }

    let mut min = count;
    for i in k..s.len() {
        if s[i] == b'W' {
            count += 1;
        }
        if s[i - k] == b'W' {
            count -= 1;
        }
        min = min.min(count);
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, minimum_recolors(b"WBBWWBBWBW", 7));
        assert_eq!(0, minimum_recolors(b"WBWBBBW", 2));
        assert_eq!(0, minimum_recolors(b"WBWBBBW", 10));
    }
}
