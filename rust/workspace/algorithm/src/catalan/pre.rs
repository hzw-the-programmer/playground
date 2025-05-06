// Time Complexity: O(n)
// Auxiliary Space: O(1)
pub fn catalan(n: usize) -> usize {
    let mut pre = 1;

    for i in 2..=n {
        pre = pre * (4 * i - 2) / (i + 1);
    }

    pre
}
