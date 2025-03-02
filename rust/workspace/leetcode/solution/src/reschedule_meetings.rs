// 3439. Reschedule Meetings for Maximum Free Time I
// 在长度为 n + 1 的空闲时常数组中，求和最大的 k + 1 子数组
// *代表会议时常，_代表空闲时常
// _*_*_*_
pub fn max_free_time(event_time: i32, k: usize, start_time: &[i32], end_time: &[i32]) -> i32 {
    let n = start_time.len();

    let get = |i| {
        if i == 0 {
            return start_time[0];
        }
        if i == n {
            return event_time - end_time[i - 1];
        }
        start_time[i] - end_time[i - 1]
    };

    let mut sum = 0;
    let mut max = 0;
    for i in 0..=n {
        sum += get(i);
        if i < k {
            continue;
        }
        max = max.max(sum);
        sum -= get(i - k);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, max_free_time(5, 1, &[1, 3], &[2, 5]));
        assert_eq!(6, max_free_time(10, 1, &[0, 2, 9], &[1, 4, 10]));
        assert_eq!(0, max_free_time(5, 2, &[0, 1, 2, 3, 4], &[1, 2, 3, 4, 5]));
    }
}
