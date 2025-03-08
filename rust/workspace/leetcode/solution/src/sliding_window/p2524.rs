// 2524. Maximum Frequency Score of a Subarray

use std::collections::HashMap;

pub fn max_frequency_score(nums: Vec<i32>, k: usize) -> i32 {
    let n = nums.len();
    let modulus = 10_i32.pow(9) + 7;
    
    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(nums[i]).or_insert(0) += 1;
    }

    let mut cur = 0;
    for (&k, &v) in map.iter() {
        cur = (cur + quick_power(k, v, modulus)) % modulus;
    }

    let mut max = cur;
    for i in k..n {
        let added = nums[i];
        let removed = nums[i-k];
        if added != removed {
            if let Some(&v) = map.get(&added) {
                cur += (added - 1) * quick_power(added, v, modulus);
            } else {
                cur += added;
            }

            let v = map.get(&removed).unwrap();
            if *v > 1 {
                cur -= (removed - 1) * quick_power(removed, *v - 1, modulus);
            } else {
                cur -= removed;
            }

            cur = (cur + modulus) % modulus;

            *map.entry(added).or_insert(0) += 1;
            *map.entry(removed).or_insert(0) -= 1;

            max = max.max(cur);
        } 
    }
    max
}

// Quick power function for calculating a^n % mod efficiently
pub fn quick_power(mut base: i32, mut exponent: i32, modulus: i32) -> i32 {
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exponent >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, max_frequency_score(vec![1,1,1,2,1,2], 3));
        assert_eq!(1, max_frequency_score(vec![1,1,1,1,1,1], 4));
    }
}