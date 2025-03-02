pub fn subarray_beauty(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![0; n - k + 1];
    let mut v = Vec::new();
    for i in 0..k {
        if nums[i] < 0 {
            v.push(nums[i]);
        }
    }
    if v.len() >= x {
        v.sort();
        res[0] = v[x - 1];
    }

    for i in k..n {
        if nums[i] < 0 {
            v.push(nums[i]);
        }
        if nums[i - k] < 0 {
            v.remove(v.iter().position(|e| *e == nums[i - k]).unwrap());
        }

        if v.len() >= x {
            v.sort();
            res[i - k + 1] = v[x - 1];
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!([-1, -2, -2][..], subarray_beauty(&[1, -1, -3, -2, 3], 3, 2));
        assert_eq!(
            [-1, -2, -3, -4][..],
            subarray_beauty(&[-1, -2, -3, -4, -5], 2, 2)
        );
        assert_eq!(
            [-3, 0, -3, -3, -3][..],
            subarray_beauty(&[-3, 1, 2, -3, 0, -3], 2, 1)
        );
    }
}
