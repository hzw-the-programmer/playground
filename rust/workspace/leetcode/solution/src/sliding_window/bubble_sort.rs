pub fn bubble_sort(nums: &mut [i32]) {
    for i in (1..nums.len()).rev() {
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
            }
        }
    }
}

// cargo test bubble_sort -p solution
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a = [5, 4, 3, 2, 1];
        let wanted = [1, 2, 3, 4, 5];
        bubble_sort(&mut a);
        assert_eq!(wanted, a);
    }
}
