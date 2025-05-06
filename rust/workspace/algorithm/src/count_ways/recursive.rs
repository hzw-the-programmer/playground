// time: O(2^n)
// space: O(n)
pub fn count_ways(n: usize) -> usize {
    // base cases: if there are 0 or 1 stairs,
    // there is only one way to reach the top.
    if n <= 1 {
        1
    } else {
        count_ways(n - 1) + count_ways(n - 2)
    }
}
