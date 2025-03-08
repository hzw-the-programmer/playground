// 683. K Empty Slots
pub fn k_empty_slots_1(bulbs: Vec<i32>, k: i32) -> i32 {
    let n = bulbs.len();
    let k = k as usize;
    let mut a = vec![0; n];
    for i in 0..n {
        let j = (bulbs[i] - 1) as usize;
        a[j] = 1;
        if (j > k && a[j - k - 1] == 1 && a[j - k..j].iter().sum::<i32>() == 0)
            || (j < n - k - 1 && a[j + k + 1] == 1 && a[j + 1..j + k].iter().sum::<i32>() == 0)
        {
            return i as i32 + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, k_empty_slots_1(vec![1, 3, 2], 1));
        assert_eq!(-1, k_empty_slots_1(vec![1, 2, 3], 1));
    }
}
