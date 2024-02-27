fn bubble_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn bubble_sort_teset() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    let wanted = [17, 20, 26, 31, 44, 54, 55, 77, 93];

    bubble_sort(&mut nums);
    assert_eq!(nums, wanted);
}
