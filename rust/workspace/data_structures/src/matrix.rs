// https://www.geeksforgeeks.org/matrix-exponentiation/

pub const MOD: i64 = 1_000_000_000 + 7;

pub fn multiply2x2(a: &[[i64; 2]; 2], b: &[[i64; 2]; 2]) -> [[i64; 2]; 2] {
    let mut c = [[0; 2]; 2];

    c[0][0] = (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % MOD;
    c[0][1] = (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % MOD;
    c[1][0] = (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % MOD;
    c[1][1] = (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % MOD;

    c
}

pub fn power2x2(a: &[[i64; 2]; 2], mut n: u32) -> [[i64; 2]; 2] {
    let mut ans = [[1, 0], [0, 1]];
    let mut t = *a;

    while n > 0 {
        if n & 1 == 1 {
            ans = multiply2x2(&ans, &t);
        }
        t = multiply2x2(&t, &t);
        n >>= 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply2x2() {
        let a = [[1, 2], [3, 4]];
        let b = [[5, 6], [7, 8]];
        let c = multiply2x2(&a, &b);
        assert_eq!(c, [[19, 22], [43, 50]]);
    }

    #[test]
    fn test_power2x2() {
        let a = [[1, 2], [3, 4]];
        let b = power2x2(&a, 2);
        assert_eq!(b, [[7, 10], [15, 22]]);
    }
}
