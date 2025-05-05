use super::word_break;

#[test]
fn test_word_break() {
    let dictionary = [
        "mobile", "samsung", "sam", "sung", "ma\n", "mango", "icecream", "and", "go", "i", "like",
        "ice", "cream",
    ];
    assert!(word_break("ilikesamsung", &dictionary));
    assert!(word_break("iiiiiiii", &dictionary));
    assert!(word_break("", &dictionary));
    assert!(word_break("ilikelikeimangoiii", &dictionary));
    assert!(word_break("samsungandmango", &dictionary));
    assert!(!word_break("samsungandmangok", &dictionary));

    let dictionary = ["leet", "code"];
    assert!(word_break("leetcode", &dictionary));

    let dictionary = ["apple", "pen"];
    assert!(word_break("applepenapple", &dictionary));

    let dictionary = ["cats", "pen", "sand", "and", "cat"];
    assert!(!word_break("catsandog", &dictionary));

    let dictionary = ["aaa", "aaaa"];
    assert!(!word_break("aaaaa", &dictionary));

    let s = "a";
    assert!(word_break(s, &["a"]));
    assert!(!word_break(s, &["b"]));

    let s = "";
    assert!(word_break(s, &[""]));
    assert!(word_break(s, &["a"]));
}
