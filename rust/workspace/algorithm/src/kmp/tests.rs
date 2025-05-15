use super::*;

#[test]
fn test_build_lps() {
    assert_eq!(build_lps("AAAA"), vec![0, 1, 2, 3]);
    assert_eq!(build_lps("ABCDE"), vec![0, 0, 0, 0, 0]);
    assert_eq!(
        build_lps("AABAACAABAA"),
        vec![0, 1, 0, 1, 2, 0, 1, 2, 3, 4, 5]
    );
    assert_eq!(build_lps("AAACAAAAAC"), vec![0, 1, 2, 0, 1, 2, 3, 3, 3, 4]);
    assert_eq!(build_lps("AAABAAA"), vec![0, 1, 2, 0, 1, 2, 3]);
}

#[test]
fn test_search() {
    assert_eq!(search("abcab", "ab"), vec![0, 3]);
    assert_eq!(search("aabaacaadaabaaba", "aaba"), vec![0, 9, 12]);
}
