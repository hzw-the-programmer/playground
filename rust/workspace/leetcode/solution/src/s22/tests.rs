use super::generate_parenthesis;

#[test]
fn test_generate_parenthesis() {
    assert_eq!(generate_parenthesis(1), vec!["()"]);
    assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
    assert_eq!(
        generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
