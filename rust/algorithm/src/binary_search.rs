fn binary_search(nums: &[i32], target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;

    let mut count = 0;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        if nums[mid] == target {
            return true;
        } else if nums[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    false
}

// cargo test -- --nocapture

#[test]
fn binary_search_test() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    assert!(binary_search(&nums, 3));
    assert!(!binary_search(&nums, 63));
}
