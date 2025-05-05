/*
  https://www.geeksforgeeks.org/word-break-problem-dp-32/
  Time Complexity: O(n * m * k), where n is the length of string and m is the number of dictionary words and k is the length of maximum sized string in dictionary.
  Space Complexity: O(n)
*/

pub fn word_break(s: &str, dictionary: &[&str]) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for w in dictionary {
            if i >= w.len() && dp[i - w.len()] && &s[i - w.len()..i] == *w {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}
