// Time Complexity: O(2^n)
// Auxiliary Space: O(n)
pub fn catalan(n: usize) -> usize {
    if n <= 1 {
        1
    } else {
        let mut res = 0;
        for i in 0..n {
            res += catalan(i) * catalan(n - i - 1);
        }
        res
    }
}
