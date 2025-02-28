// 1151. Minimum Swaps to Group All 1’s Together
pub fn minimum_swaps(data: &[i32]) -> i32 {
    let k = data.iter().filter(|&&x| x == 1).count();
    if k <= 1 {
        return 0;
    }

    let mut count = 0;
    for i in 0..k {
        if data[i] == 0 {
            count += 1;
        }
    }

    let mut min = count;
    for i in k..data.len() {
        if data[i] == 0 {
            count += 1;
        }
        if data[i - k] == 0 {
            count -= 1;
        }
        min = min.min(count);
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, minimum_swaps(&[1, 0, 1, 0, 1]));
        assert_eq!(0, minimum_swaps(&[0, 0, 0, 1, 0]));
        assert_eq!(3, minimum_swaps(&[1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1]));
    }
}
