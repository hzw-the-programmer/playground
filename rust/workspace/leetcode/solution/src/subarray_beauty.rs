/*
   2653. Sliding Subarray Beauty

   Given an integer array nums containing n integers, find the beauty of each subarray of size k.
   The beauty of a subarray is the xth smallest integer in the subarray if it is negative, or 0 if there are fewer than x negative integers.
   Return an integer array containing n - k + 1 integers, which denote the beauty of the subarrays in order from the first index in the array.
   * A subarray is a contiguous non-empty sequence of elements within an array.

   Constraints:
   * n == nums.length
   * 1 <= n <= 105
   * 1 <= k <= n
   * 1 <= x <= k
   * -50 <= nums[i] <= 50
*/

pub fn subarray_beauty(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![0; n - k + 1];
    let mut count = [0; 101];
    for i in 0..k {
        count[(nums[i] + 50) as usize] += 1;
    }
    res[0] = find_xth_smallest(&count, x);

    for i in k..n {
        count[(nums[i] + 50) as usize] += 1;
        count[(nums[i - k] + 50) as usize] -= 1;
        res[i - k + 1] = find_xth_smallest(&count, x);
    }

    res
}

fn find_xth_smallest(count: &[u8], x: usize) -> i32 {
    let mut sum = 0;
    for i in 0..50 {
        sum += count[i] as usize;
        if sum >= x {
            return i as i32 - 50;
        }
    }
    0
}

pub fn subarray_beauty_2(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
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
