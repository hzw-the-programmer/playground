pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m > 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, gcd(14, 15));
        assert_eq!(3 * 11, gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19));
    }
}
