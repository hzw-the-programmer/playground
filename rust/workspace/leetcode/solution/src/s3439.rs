// 3439. Reschedule Meetings for Maximum Free Time I

pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let n = start_time.len();
    let mut free = vec![0; n + 1];
    let mut pre_end = 0;
    for i in 0..n {
        free[i] = start_time[i] - pre_end;
        pre_end = end_time[i];
    }
    free[n] = event_time - pre_end;

    let n = free.len();
    let k = k as usize + 1;
    let mut sum = 0;
    let mut ans = 0;
    for i in 0..n {
        sum += free[i];

        if i < k - 1 {
            continue;
        }

        ans = ans.max(sum);

        sum -= free[i + 1 - k];
    }

    ans
}

pub fn max_free_time_v2(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let n = start_time.len();
    let get = |i| {
        if i == 0 {
            start_time[0]
        } else if i == n {
            event_time - end_time[n - 1]
        } else {
            start_time[i] - end_time[i - 1]
        }
    };

    let k = k as usize + 1;
    let mut sum = 0;
    let mut ans = 0;
    for i in 0..=n {
        sum += get(i);

        if i < k - 1 {
            continue;
        }

        ans = ans.max(sum);

        sum -= get(i + 1 - k);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(f: fn(i32, i32, Vec<i32>, Vec<i32>) -> i32) {
        assert_eq!(f(5, 1, vec![1, 3], vec![2, 5]), 2);
        assert_eq!(f(10, 1, vec![0, 2, 9], vec![1, 4, 10]), 6);
        assert_eq!(f(5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_max_free_time() {
        test(max_free_time);
    }

    #[test]
    fn test_max_free_time_v2() {
        test(max_free_time_v2);
    }
}
