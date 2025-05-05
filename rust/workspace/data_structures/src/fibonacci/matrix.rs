// https://www.geeksforgeeks.org/matrix-exponentiation/
// time: O(log(n))
// space: O(1)
pub fn fibonacci(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        let mut m = [[1, 1], [1, 0]];
        m = crate::matrix::power2x2(&m, (n - 1) as u32);
        let initial = [[1, 0], [0, 0]];
        let result = crate::matrix::multiply2x2(&m, &initial);
        (result[0][0] % crate::matrix::MOD) as _
    }
}
