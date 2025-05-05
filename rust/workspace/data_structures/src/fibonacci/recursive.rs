// time: O(2^n)
// space: O(n)
pub fn fibonacci(n: usize) -> usize {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
