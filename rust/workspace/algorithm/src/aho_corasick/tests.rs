use super::*;

// #[test]
// fn test_starts_with() {
//     let mut am = ACAutomaton::new();
//     am.build(&["ab", "about", "at", "ate", "be", "bed", "edge", "get"]);
//     assert!(am.starts_with("ab"));
//     assert!(!am.starts_with("abz"));

//     assert!(am.starts_with("about"));
//     assert!(!am.starts_with("aboutz"));

//     assert!(am.starts_with("at"));
//     assert!(!am.starts_with("atz"));

//     assert!(am.starts_with("ate"));
//     assert!(!am.starts_with("atez"));

//     assert!(am.starts_with("be"));
//     assert!(!am.starts_with("bez"));

//     assert!(am.starts_with("bed"));
//     assert!(!am.starts_with("bedz"));

//     assert!(am.starts_with("edge"));
//     assert!(!am.starts_with("edgez"));

//     assert!(am.starts_with("get"));
//     assert!(!am.starts_with("getz"));
// }

#[test]
fn test_seach() {
    let words = ["his", "he", "she", "hers"];
    let mut am = ACAutomaton::new();
    am.build(&words);
    let result = am.search("ahishers", &words);
    assert_eq!(result[0], (0, 1, 3));
    assert_eq!(result[1], (1, 4, 5));
    assert_eq!(result[2], (2, 3, 5));
    assert_eq!(result[3], (3, 4, 7));
}

// #[test]
// fn test_seach() {
//     let words = ["ab", "be", "bed"];
//     let mut am = ACAutomaton::new();
//     am.build(&words);
//     // let result = am.search("ahishers", &words);
//     // assert_eq!(result[0], (0, 1, 3));
// }
