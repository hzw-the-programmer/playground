// 2269. Find the K-Beauty of a Number
pub fn divisor_substrs(num: i32, k: usize) -> i32 {
    let mut n = num;

    let mut pk = 1;
    for _ in 0..k {
        pk *= 10;
    }

    let mut count = 0;
    while n >= pk / 10 {
        let r = n % pk;
        if r != 0 && num % r == 0 {
            count += 1;
        }
        n /= 10;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, divisor_substrs(240, 2));
        assert_eq!(2, divisor_substrs(430043, 2));
    }
}
