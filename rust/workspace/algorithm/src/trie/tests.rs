use super::Trie;

#[test]
fn test_insert_and_search() {
    let mut trie = Trie::new();
    trie.insert("hello");
    assert!(trie.search("hello"));
    assert!(!trie.search("he"));
    assert!(!trie.search("world"));
}

#[test]
fn test_starts_with() {
    let mut trie = Trie::new();
    trie.insert("apple");
    assert!(trie.starts_with("app"));
    assert!(!trie.starts_with("ban"));
}

#[test]
fn test_empty_trie() {
    let trie = Trie::new();
    assert!(!trie.search("anyword"));
    assert!(!trie.starts_with("anyprefix"));
}

#[test]
fn test_insert_multible_words() {
    let mut trie = Trie::new();
    trie.insert("cat");
    trie.insert("dog");
    trie.insert("cattle");
    assert!(trie.search("cat"));
    assert!(trie.search("dog"));
    assert!(trie.search("cattle"));
    assert!(!trie.search("rat"));
    assert!(trie.starts_with("ca"));
}
