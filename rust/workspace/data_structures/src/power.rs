// https://www.geeksforgeeks.org/fast-exponention-using-bit-manipulation/

// time: O(n)
// space: O(1)
pub fn power(a: u64, n: u64) -> u64 {
    let mut ans = 1;

    for _ in 0..n {
        ans *= a;
    }

    ans
}

// time: O(log(n))
// space: O(1)
pub fn power_v2(mut a: u64, mut n: u64) -> u64 {
    let mut ans = 1;

    while n > 0 {
        if n & 1 == 1 {
            ans *= a;
        }
        a = a * a;
        n >>= 1;
    }

    ans
}

// time: O(log(n))
// space:: O(1)
pub fn power_recursive(a: u64, n: u64) -> u64 {
    if n == 0 {
        // base case: a^0 = 1
        1
    } else if n % 2 == 0 {
        let t = power_recursive(a, n / 2);
        t * t
    } else {
        a * power_recursive(a, n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(f: fn(u64, u64) -> u64) {
        assert_eq!(f(3, 5), 243);
    }

    #[test]
    fn test_power() {
        test(power);
        test(power_v2);
        test(power_recursive);
    }
}
