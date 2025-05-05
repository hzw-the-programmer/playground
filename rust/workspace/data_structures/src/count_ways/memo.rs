// time: O(n)
// space: O(n)
pub fn count_ways(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    count_ways_recursive(n, &mut memo)
}

fn count_ways_recursive(n: usize, memo: &mut [usize]) -> usize {
    if n <= 1 {
        1
    } else {
        if memo[n] == 0 {
            memo[n] = count_ways_recursive(n - 1, memo) + count_ways_recursive(n - 2, memo);
        }
        memo[n]
    }
}
