// Time Complexity: O(n)
// Auxiliary Space: O(1)
pub fn catalan(n: usize) -> usize {
    binomial_coefficient(2 * n, n) / (n + 1)
}

fn binomial_coefficient(n: usize, mut k: usize) -> usize {
    let mut res = 1;

    if k > n - k {
        k = n - k;
    }

    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }

    res
}
