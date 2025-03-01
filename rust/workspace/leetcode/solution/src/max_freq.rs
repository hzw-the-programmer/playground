use std::collections::HashMap;

pub fn max_freq(s: &str, max_letters: usize, min_size: usize, _max_size: usize) -> usize {
    let s = s.as_bytes();
    let n = s.len();
    let k = min_size;
    let mut map = HashMap::new();

    let mut letters = HashMap::new();
    for i in 0..k {
        *letters.entry(s[i]).or_insert(0) += 1;
    }

    let mut max = 0;
    if letters.len() <= max_letters {
        *map.entry(&s[0..k]).or_insert(0) += 1;
        max = 1;
    }

    for i in k..n {
        *letters.entry(s[i]).or_insert(0) += 1;
        let v = letters.get_mut(&s[i - k]).unwrap();
        *v -= 1;
        if *v == 0 {
            letters.remove(&s[i - k]);
        }

        if letters.len() <= max_letters {
            let e = map.entry(&s[i - k + 1..=i]).or_insert(0);
            *e += 1;
            max = max.max(*e);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, max_freq("aababcaab", 2, 3, 4));
        assert_eq!(2, max_freq("aaaa", 1, 3, 3));
    }
}
