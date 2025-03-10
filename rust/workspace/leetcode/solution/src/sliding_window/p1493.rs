// 1493. Longest Subarray of 1's After Deleting One Element

/*
  a  : 0, 1, 1, 1, 0, 1, 1, 0, 1
       --->
  pre: 0, 1, 2, 3, 0, 1, 2, 0, 1
                            <---
  suf: 0, 3, 2, 1, 0, 2, 1, 0, 1
*/
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut pre = vec![0; n];
    pre[0] = nums[0];
    for i in 1..n {
        pre[i] = if nums[i] == 0 { 0 } else { pre[i - 1] + 1 };
    }

    let mut suf = vec![0; n];
    suf[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        suf[i] = if nums[i] == 0 { 0 } else { suf[i + 1] + 1 };
    }

    let mut res = 0;
    for i in 0..n {
        let pre_ones = if i == 0 { 0 } else { pre[i - 1] };
        let suf_ones = if i == n - 1 { 0 } else { suf[i + 1] };
        res = res.max(pre_ones + suf_ones);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, longest_subarray(vec![1, 1, 0, 1]));
        assert_eq!(5, longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]));
        assert_eq!(2, longest_subarray(vec![1, 1, 1]));
    }
}
