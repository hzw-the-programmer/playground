// 1610. Maximum Number of Visible Points

struct Solution;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut same_ctn = 0;

        let mut radians = Vec::new();
        for point in points.iter() {
            if *point == location {
                same_ctn += 1;
                continue;
            }
            let y = (point[1] - location[1]) as f64;
            let x = (point[0] - location[0]) as f64;
            radians.push(y.atan2(x));
        }

        radians.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let n = radians.len();
        for i in 0..n {
            radians.push(radians[i] + std::f64::consts::TAU);
        }

        let angle = (angle as f64).to_radians();
        let mut max_ctn = 0;
        let mut right = 0;
        for left in 0..n {
            while right < 2 * n && radians[right] <= radians[left] + angle {
                right += 1;
            }
            max_ctn = max_ctn.max(right - left);
        }

        same_ctn + max_ctn as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            3,
            Solution::visible_points(
                [[2, 1], [2, 2], [3, 3]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<_>>>(),
                90,
                vec![1, 1]
            )
        );
        assert_eq!(
            4,
            Solution::visible_points(
                [[2, 1], [2, 2], [3, 4], [1, 1]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<_>>>(),
                90,
                vec![1, 1]
            )
        );
        assert_eq!(
            1,
            Solution::visible_points(
                [[1, 0], [2, 1]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<_>>>(),
                13,
                vec![1, 1]
            )
        );
    }
}
