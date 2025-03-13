// 904. Fruit Into Baskets

use std::collections::HashMap;

/*
  最长子串的长度，子串最多两个不同的元素
*/
pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut l = 0;
    let mut res = 0;
    for r in 0..fruits.len() {
        *map.entry(fruits[r]).or_insert(0) += 1;
        while map.len() > 2 {
            let c = map.get_mut(&fruits[l]).unwrap();
            *c -= 1;
            if *c == 0 {
                map.remove(&fruits[l]);
            }
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, total_fruit(vec![1, 2, 1]));
        assert_eq!(3, total_fruit(vec![0, 1, 2, 2]));
        assert_eq!(4, total_fruit(vec![1, 2, 3, 2, 2]));
    }
}
