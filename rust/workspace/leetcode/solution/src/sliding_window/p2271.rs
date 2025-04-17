// 2271. Maximum White Tiles Covered by a Carpet

struct Solution;

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles;
        tiles.sort_by_key(|v| v[0]);

        let mut left = 0;
        let mut covered = 0;
        let mut res = 0;

        for p in tiles.iter() {
            covered += p[1] - p[0] + 1;

            while tiles[left][0] + carpet_len - 1 < p[0] {
                covered -= tiles[left][1] - tiles[left][0] + 1;
                left += 1;
            }

            let uncovered = if tiles[left][0] + carpet_len - 1 < p[1] {
                p[1] - (tiles[left][0] + carpet_len - 1)
            } else {
                0
            };

            res = res.max(covered - uncovered);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_white_tile() {
        assert_eq!(
            Solution::maximum_white_tiles(
                vec![
                    vec![1, 5],
                    vec![10, 11],
                    vec![12, 18],
                    vec![20, 25],
                    vec![30, 32]
                ],
                10
            ),
            9
        );

        assert_eq!(
            Solution::maximum_white_tiles(vec![vec![10, 11], vec![1, 1]], 2),
            2
        );
    }
}
