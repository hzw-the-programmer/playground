// 2379. Minimum Recolors to Get K Consecutive Black Blocks

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let blocks = blocks.as_bytes();
    let k = k as usize;
    let mut count = blocks[0..k].iter().filter(|&&b| b == b'W').count();
    let mut ans = count;
    for i in k..blocks.len() {
        if blocks[i] == b'W' {
            count += 1;
        }
        if blocks[i - k] == b'W' {
            count -= 1;
        }
        ans = ans.min(count);
    }
    ans as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_recolors() {
        assert_eq!(3, minimum_recolors("WBBWWBBWBW".to_string(), 7));
        assert_eq!(0, minimum_recolors("WBWBBBW".to_string(), 2));
    }
}
