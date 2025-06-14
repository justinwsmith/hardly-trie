#![cfg(test)]

use hardly_trie::trie::Trie;

#[test]
fn test_shared_prefix() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Insert two keys that share a prefix
    trie.insert("apple", "apple".to_string());
    trie.insert("app", "app".to_string());

    // Check that both keys are present
    assert_eq!(trie.get("apple"), Some(&"apple".to_string()));
    assert_eq!(trie.get("app"), Some(&"app".to_string()));
    assert_eq!(trie.len(), 2);

    // Delete the longer key
    assert_eq!(trie.delete("apple"), Some("apple".to_string()));

    // Check that the shorter key is still present
    assert_eq!(trie.get("apple"), None);
    assert_eq!(trie.get("app"), Some(&"app".to_string()));
    assert_eq!(trie.len(), 1);

    // Delete the shorter key
    assert_eq!(trie.delete("app"), Some("app".to_string()));

    // Check that the trie is empty
    assert_eq!(trie.get("app"), None);
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());
}

#[test]
fn test_shared_prefix_reversed_deletion() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Insert two keys that share a prefix
    trie.insert("apple", "apple".to_string());
    trie.insert("app", "app".to_string());

    // Check that both keys are present
    assert_eq!(trie.get("apple"), Some(&"apple".to_string()));
    assert_eq!(trie.get("app"), Some(&"app".to_string()));
    assert_eq!(trie.len(), 2);

    // Delete the shorter key
    assert_eq!(trie.delete("app"), Some("app".to_string()));

    // Check that the longer key is still present
    assert_eq!(trie.get("app"), None);
    assert_eq!(trie.get("apple"), Some(&"apple".to_string()));
    assert_eq!(trie.len(), 1);

    // Delete the longer key
    assert_eq!(trie.delete("apple"), Some("apple".to_string()));

    // Check that the trie is empty
    assert_eq!(trie.get("apple"), None);
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());
}
