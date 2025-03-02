pub fn counting_sort(nums: &mut [i32]) {
    let mut min = nums[0];
    let mut max = nums[0];
    for &e in nums.iter() {
        if min > e {
            min = e;
        }
        if max < e {
            max = e;
        }
    }

    let range = (max - min + 1) as usize;
    let mut count = vec![0; range];
    for &e in nums.iter() {
        count[(e - min) as usize] += 1;
    }

    let mut j = 0;
    for (i, &c) in count.iter().enumerate() {
        for _ in 0..c {
            nums[j] = i as i32 + min;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a = [5, 2, 3, 1];
        counting_sort(&mut a);
        assert_eq!([1, 2, 3, 5], a);

        let mut a = [5, 1, 1, 2, 0, 0];
        counting_sort(&mut a);
        assert_eq!([0, 0, 1, 1, 2, 5], a);
    }
}
