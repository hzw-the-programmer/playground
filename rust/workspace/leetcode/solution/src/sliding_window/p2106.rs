// 2106. Maximum Fruits Harvested After at Most K Steps

struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let len = fruits.len();

        // 这个居然要快点？
        let mut left = 0;
        while left < len && fruits[left][0] < start_pos - k {
            left += 1;
        }
        // let mut left = fruits.binary_search_by_key(&(start_pos - k), |p| p[0]).unwrap_or_else(|e| e);
        // if left == len {
        //     return 0;
        // }

        let mut sum = 0;

        let mut right = left;
        while right < len && fruits[right][0] <= start_pos {
            sum += fruits[right][1];
            right += 1;
        }
        if right == len {
            return sum;
        }

        let mut res = sum;

        while right < len && fruits[right][0] <= start_pos + k {
            sum += fruits[right][1];
            while fruits[left][0] < start_pos
                && (fruits[right][0] - fruits[left][0]) + (start_pos - fruits[left][0]) > k
                && (fruits[right][0] - fruits[left][0]) + (fruits[right][0] - start_pos) > k
            {
                sum -= fruits[left][1];
                left += 1;
            }
            right += 1;
            res = res.max(sum);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_total_fruits_1() {
        assert_eq!(
            9,
            Solution::max_total_fruits(
                [[2, 8], [6, 3], [8, 6]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                4
            )
        );

        assert_eq!(
            14,
            Solution::max_total_fruits(
                [[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                4
            )
        );

        assert_eq!(
            0,
            Solution::max_total_fruits(
                [[0, 3], [6, 4], [8, 5]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                3,
                2
            )
        );
    }

    #[test]
    fn test_max_total_fruits_2() {
        // 0 1 2 3 4
        // 8   3
        assert_eq!(
            0,
            Solution::max_total_fruits(
                [[0, 8], [2, 3]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                4,
                1
            )
        );

        // 0 1 2 3 4 5
        // 8   3 4
        assert_eq!(
            4,
            Solution::max_total_fruits(
                [[0, 8], [2, 3], [3, 4]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                2
            )
        );

        // 0 1 2 3 4 5 6
        // 8   3 4     5
        assert_eq!(
            5,
            Solution::max_total_fruits(
                [[0, 8], [2, 3], [3, 4], [6, 5]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                2
            )
        );

        // 0 1 2 3 4 5 6 7
        // 8   3 4     1 4
        assert_eq!(
            5,
            Solution::max_total_fruits(
                [[0, 8], [2, 3], [3, 4], [6, 1], [7, 4]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                2
            )
        );

        // 0 1 2 3 4 5 6 7 8 9
        // 8         1 1 4   1
        assert_eq!(
            7,
            Solution::max_total_fruits(
                [[0, 8], [5, 1], [6, 1], [7, 4], [9, 1]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                4
            )
        );

        // 0 1 2 3 4 5 6 7 8 9
        // 8           1 4   1
        assert_eq!(
            6,
            Solution::max_total_fruits(
                [[0, 8], [6, 1], [7, 4], [9, 1]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                4
            )
        );

        // 0 1 2 3 4 5 6 7 8 9
        // 8     2 2   2 1   1
        assert_eq!(
            6,
            Solution::max_total_fruits(
                [[0, 8], [3, 2], [4, 2], [6, 2], [7, 1], [9, 1]]
                    .iter()
                    .map(|p| vec![p[0], p[1]])
                    .collect::<Vec<Vec<i32>>>(),
                5,
                4
            )
        );
    }
}
