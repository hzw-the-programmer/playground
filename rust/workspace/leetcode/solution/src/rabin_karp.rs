pub fn rabin_karp(text: &str, pattern: &str) -> bool {
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    let n = text.len();
    let k = pattern.len();
    if k > n {
        return false;
    }

    let b = 256;
    let q = 101;
    let mut t = 0;
    let mut p = 0;
    for i in 0..k {
        t = (t * b + text[i] as usize) % q;
        p = (p * b + pattern[i] as usize) % q;
    }

    let mut h = 1;
    for _ in 0..k - 1 {
        h = (h * b) % q;
    }
    for i in 0..=n - k {
        if t == p && &text[i..i + k] == pattern {
            return true;
        }

        t = ((t + q - (text[i] as usize * h) % q) * b + text[i + k] as usize) % q;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(rabin_karp("ABABDABACDABABCABAB", "ABABCABAB"));
    }
}
