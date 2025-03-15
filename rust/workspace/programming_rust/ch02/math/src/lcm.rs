use super::gcd;

pub fn lcm(m: u64, n: u64) -> u64 {
    m * n / gcd(m, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(35, lcm(5, 7));
        assert_eq!(12, lcm(4, 6));
        assert_eq!(36, lcm(12, 18));
    }
}
