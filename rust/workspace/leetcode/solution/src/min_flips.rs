// 1888. Minimum Number of Flips to Make the Binary String Alternating
// https://algo.monster/liteproblems/1888
pub fn min_flips(s: &str) -> usize {
    let s = s.as_bytes();
    let n = s.len();
    let target = b"01";

    let mut flips = 0;
    for (i, &c) in s.iter().enumerate() {
        if c != target[i & 1] {
            flips += 1;
        }
    }
    let mut min = flips.min(n - flips);

    if n % 2 == 0 {
        return min;
    }

    for (i, &c) in s.iter().enumerate() {
        // If we remove a character that needs flipping from the start, decrease the flip count
        // 在原来的位置不满足要求，表明被 flip 过，所以要 -1
        if c != target[i & 1] {
            flips -= 1;
        }
        // 在新位置不满足要求，要 +1
        if c != target[(i + n) & 1] {
            flips += 1;
        }
        min = min.min(flips).min(n - flips);
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
