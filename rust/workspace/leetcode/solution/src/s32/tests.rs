use super::longest_valid_parentheses;

#[test]
fn test_longest_valid_parentheses() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(longest_valid_parentheses("".to_string()), 0);

    assert_eq!(longest_valid_parentheses("())".to_string()), 2);
}
