// 1176. Diet Plan Performance

pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
    let n = calories.len();
    let k = k as usize;

    let mut ans = 0;
    let mut sum = 0;
    for i in 0..n {
        sum += calories[i];
        if i < k - 1 {
            continue;
        }
        if sum < lower {
            ans -= 1;
        } else if sum > upper {
            ans += 1;
        }
        sum -= calories[i + 1 - k];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diet_plan_performance() {
        assert_eq!(diet_plan_performance(vec![1, 2, 3, 4, 5], 1, 3, 3), 0);
        assert_eq!(diet_plan_performance(vec![3, 2], 2, 0, 1), 1);
        assert_eq!(diet_plan_performance(vec![6, 5, 0, 0], 2, 1, 5), 0);
    }
}
