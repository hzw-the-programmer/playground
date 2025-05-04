// recursion
// time: O(2^n)
// space: O(n)
pub fn count_ways(n: usize) -> usize {
    // base cases: if there are 0 or 1 stairs,
    // there is only one way to reach the top.
    if n == 0 || n == 1 {
        1
    } else {
        count_ways(n - 1) + count_ways(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ways() {
        assert_eq!(count_ways(0), 1);
        assert_eq!(count_ways(1), 1);
        assert_eq!(count_ways(2), 2);
        assert_eq!(count_ways(3), 3);
        assert_eq!(count_ways(4), 5);
        assert_eq!(count_ways(5), 8);
    }
}
