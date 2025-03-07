// 1016. Binary String With Substrings Representing 1 To N
pub fn query_string_1(s: String, n: i32) -> bool {
    for i in 1..=n {
        if !s.contains(&format!("{i:b}")) {
            return false;
        }
    }
    true
}

use std::collections::HashSet;

pub fn query_string_2(s: String, n: i32) -> bool {
    let mut set = HashSet::new();
    for (i, b) in s.bytes().enumerate() {
        if b - b'0' == 0 {
            continue;
        }
        let mut x = 0;
        for b in s.bytes().skip(i) {
            let b = (b - b'0') as i32;
            x = (x << 1) + b;
            if x > n {
                break;
            }
            set.insert(x);
        }
    }
    set.len() == n as usize
}

/*
  [1..n]
  假设 n 的二进制为 k=4 位
  [1..xxxx]

  第一部分：
  [1..1000) [1000..xxxx]
  [1000, 1001, 1010, 1011..xxxx]
  [2^(k-1)..n]
  总共 n-2^(k-1)+1 个数，每个数二进制位长 k
  1000
   1001
    1010
     ..
  s 长度至少要 (n-2^(k-1)+1) + (k-1)

  第二部分：
  [1, 10, 11] [100..111] [1000..xxxx]
  [100, 101, 110, 111]
  [2^(k-2)..2^(k-1)-1]
  总共 2^(k-1)-1 - 2^(k-2) + 1 个数，每个数二进制位长 k-1
  s 长度至少要 (2^(k-1)-1 - 2^(k-2) + 1) + ((k-1) - 1)

  [2^(k-1)..n] 右移 1 位为 [2^(k-2)..n/2]
  [100, 100, 101, 101..xxx]
  也就是说 [2^(k-1)..n] 这些数字的子串包含了 [2^(k-2)..n/2]
  所以第二部分只需从 [n/2+1..2^(k-1)-1]
*/
pub fn query_string_3(s: String, n: i32) -> bool {
    let k = (32 - n.leading_zeros()) as i32;

    if (s.len() as i32)
        < (n - (1 << (k - 1)) + 1 + (k - 1))
            .max(((1 << (k - 1)) - 1) - (1 << (k - 2)) + 1 + ((k - 1) - 1))
    {
        return false;
    }

    check(s.as_bytes(), k - 1, n / 2 + 1, (1 << (k - 1)) - 1)
        && check(s.as_bytes(), k, 1 << (k - 1), n)
}

fn check(s: &[u8], k: i32, l: i32, h: i32) -> bool {
    let mut set = HashSet::new();
    let mut x = 0;
    let mask = (1 << k) - 1;
    for i in 0..s.len() {
        x = ((x << 1) + (s[i] - b'0') as i32) & mask;
        if i as i32 >= k - 1 {
            if x >= l && x <= h {
                set.insert(x);
            }
        }
    }
    set.len() as i32 == h - l + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, query_string_1("0110".to_string(), 3));
        assert_eq!(false, query_string_1("0110".to_string(), 4));

        assert_eq!(true, query_string_2("0110".to_string(), 3));
        assert_eq!(false, query_string_2("0110".to_string(), 4));

        assert_eq!(true, query_string_3("0110".to_string(), 3));
        assert_eq!(false, query_string_3("0110".to_string(), 4));
    }
}
