// https://www.geeksforgeeks.org/kmp-algorithm-for-pattern-searching/

#[cfg(test)]
mod tests;

pub fn search(txt: &str, pat: &str) -> Vec<usize> {
    let lps = build_lps(pat);
    let txt = txt.as_bytes();
    let pat = pat.as_bytes();
    let mut ans = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < txt.len() {
        if txt[i] == pat[j] {
            i += 1;
            j += 1;
            if j == pat.len() {
                ans.push(i - pat.len());
                j = lps[j - 1];
            }
        } else {
            if j == 0 {
                i += 1;
            } else {
                j = lps[j - 1];
            }
        }
    }
    ans
}

fn build_lps(pat: &str) -> Vec<usize> {
    let n = pat.len();
    let pat = pat.as_bytes();
    let mut lps = vec![0; n];

    let mut len = 0;
    lps[0] = 0;

    let mut i = 1;
    while i < n {
        if pat[i] == pat[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len == 0 {
                lps[i] = 0;
                i += 1;
            } else {
                len = lps[len - 1];
            }
        }
    }

    lps
}
