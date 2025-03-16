// 2779. Maximum Beauty of an Array After Applying Operation

pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut map = std::collections::HashMap::new();
    for &i in &nums {
        for j in i - k..=i + k {
            let c = map.entry(j).or_insert(0);
            *c += 1;
            res = res.max(*c);
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(maximum_beauty(vec![4, 6, 1, 2], 2), 3);
        assert_eq!(maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
}
