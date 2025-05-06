// time: O(n*m)
// space: O(n*m)
pub fn max_gold<const N: usize, const M: usize>(mat: &[[u32; M]; N]) -> u32 {
    let mut memo = [[u32::MAX; M]; N];
    let mut ans = 0;
    for i in 0..N {
        ans = ans.max(max_gold_recursive(i, 0, mat, &mut memo));
    }
    ans
}

fn max_gold_recursive<const N: usize, const M: usize>(
    i: usize,
    j: usize,
    mat: &[[u32; M]; N],
    memo: &mut [[u32; M]; N],
) -> u32 {
    if i == N || j == M {
        0
    } else {
        if memo[i][j] == u32::MAX {
            let mut ans = 0;

            if i > 0 {
                ans = ans.max(max_gold_recursive(i - 1, j + 1, mat, memo));
            }
            ans = ans.max(max_gold_recursive(i, j + 1, mat, memo));
            ans = ans.max(max_gold_recursive(i + 1, j + 1, mat, memo));

            memo[i][j] = mat[i][j] + ans;
        }
        memo[i][j]
    }
}
