use std::collections::HashMap;

// 2107. Number of Unique Flavors After Sharing K Candies
pub fn max_flavors(candies: &[i32], k: usize) -> usize {
    let mut map = HashMap::new();

    candies
        .iter()
        .for_each(|e| *map.entry(*e).or_insert(0) += 1);

    candies[0..k].iter().for_each(|e| {
        let c = map.get_mut(e).unwrap();
        *c -= 1;
        if *c == 0 {
            map.remove(e);
        }
    });

    let mut max = map.len();

    for i in k..candies.len() {
        *map.entry(candies[i - k]).or_insert(0) += 1;

        let c = map.get_mut(&candies[i]).unwrap();
        *c -= 1;
        if *c == 0 {
            map.remove(&candies[i]);
        }

        max = max.max(map.len());
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, max_flavors(&[1, 2, 2, 3, 4, 3], 3));
        assert_eq!(2, max_flavors(&[2, 2, 2, 2, 3, 3], 2));
        assert_eq!(3, max_flavors(&[2, 4, 5], 0));
    }
}
