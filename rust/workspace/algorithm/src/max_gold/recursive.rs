// time: O(3^(n*m))
// space: O(n*m)
pub fn max_gold<const N: usize, const M: usize>(mat: &[[u32; M]; N]) -> u32 {
    let mut ans = 0;

    for i in 0..N {
        ans = ans.max(max_gold_recursive(i, 0, mat));
    }

    ans
}

fn max_gold_recursive<const N: usize, const M: usize>(
    i: usize,
    j: usize,
    mat: &[[u32; M]; N],
) -> u32 {
    if i == N || j == M {
        0
    } else {
        let mut ans = 0;

        if i > 0 {
            ans = ans.max(max_gold_recursive(i - 1, j + 1, mat));
        }
        ans = ans.max(max_gold_recursive(i, j + 1, mat));
        ans = ans.max(max_gold_recursive(i + 1, j + 1, mat));

        mat[i][j] + ans
    }
}
