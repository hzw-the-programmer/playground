pub fn counting_sort(nums: &mut [i32]) {
    let mut min = nums[0];
    let mut max = nums[0];
    for i in 1..nums.len() {
        if min > nums[i] {
            min = nums[i];
        }
        if max < nums[i] {
            max = nums[i];
        }
    }

    let range = (max - min) as usize + 1;
    let mut count = vec![0; range];
    for i in 0..nums.len() {
        count[(nums[i] - min) as usize] += 1;
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
