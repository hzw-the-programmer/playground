use super::longest_palindrome;

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab");
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
}
