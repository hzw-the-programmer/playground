use super::longest_palindrome;

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab");
    // assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    assert_eq!(longest_palindrome("12321012321".to_string()), "bab");
}
