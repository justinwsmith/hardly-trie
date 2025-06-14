#![cfg(test)]

use hardly_trie::trie::Trie;

/// Test that verifies the trie properly cleans up intermediate nodes
/// that should be removed during deletion operations.
/// This test would fail if child_remove was changed to child_mut in the delete function.
#[test]
fn test_node_cleanup_after_deletion() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Create a scenario where we have a chain of nodes that should be cleaned up
    // Insert "abc" and "abcdef"
    trie.insert("abc", "abc".to_string());
    trie.insert("abcdef", "abcdef".to_string());

    assert_eq!(trie.len(), 2);
    assert_eq!(trie.get("abc"), Some(&"abc".to_string()));
    assert_eq!(trie.get("abcdef"), Some(&"abcdef".to_string()));

    // Delete "abcdef" - this should clean up the "def" part of the trie
    assert_eq!(trie.delete("abcdef"), Some("abcdef".to_string()));
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("abc"), Some(&"abc".to_string()));
    assert_eq!(trie.get("abcdef"), None);

    // Now insert a different extension to verify the cleanup worked
    // If cleanup didn't work properly, there might be leftover nodes
    trie.insert("abcxyz", "abcxyz".to_string());
    assert_eq!(trie.len(), 2);
    assert_eq!(trie.get("abc"), Some(&"abc".to_string()));
    assert_eq!(trie.get("abcxyz"), Some(&"abcxyz".to_string()));
    assert_eq!(trie.get("abcdef"), None);
}

/// Test deletion of a key that should trigger cleanup of a long chain
#[test]
fn test_long_chain_cleanup() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Insert a single long key
    trie.insert("verylongkeywithnobranches", "value".to_string());
    assert_eq!(trie.len(), 1);
    assert_eq!(
        trie.get("verylongkeywithnobranches"),
        Some(&"value".to_string())
    );

    // Delete it - this should clean up the entire chain since there are no branches
    assert_eq!(
        trie.delete("verylongkeywithnobranches"),
        Some("value".to_string())
    );
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());

    // Verify we can still use the trie normally after cleanup
    trie.insert("newkey", "newvalue".to_string());
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("newkey"), Some(&"newvalue".to_string()));
}

/// Test that intermediate nodes with values are preserved during cleanup
#[test]
fn test_preserve_intermediate_nodes_with_values() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Create a scenario: "app", "apple", "application"
    trie.insert("app", "app".to_string());
    trie.insert("apple", "apple".to_string());
    trie.insert("application", "application".to_string());

    assert_eq!(trie.len(), 3);

    // Delete "application" - should not affect "app" or "apple"
    assert_eq!(trie.delete("application"), Some("application".to_string()));
    assert_eq!(trie.len(), 2);
    assert_eq!(trie.get("app"), Some(&"app".to_string()));
    assert_eq!(trie.get("apple"), Some(&"apple".to_string()));
    assert_eq!(trie.get("application"), None);

    // Delete "apple" - should not affect "app"
    assert_eq!(trie.delete("apple"), Some("apple".to_string()));
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("app"), Some(&"app".to_string()));
    assert_eq!(trie.get("apple"), None);

    // Delete "app" - should leave trie empty
    assert_eq!(trie.delete("app"), Some("app".to_string()));
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());
}

/// Test cleanup behavior with multiple branches
#[test]
fn test_cleanup_with_multiple_branches() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Create a branching structure
    trie.insert("cat", "cat".to_string());
    trie.insert("car", "car".to_string());
    trie.insert("card", "card".to_string());
    trie.insert("care", "care".to_string());

    assert_eq!(trie.len(), 4);

    // Delete "card" - should not affect the branch point at "car"
    assert_eq!(trie.delete("card"), Some("card".to_string()));
    assert_eq!(trie.len(), 3);
    assert_eq!(trie.get("cat"), Some(&"cat".to_string()));
    assert_eq!(trie.get("car"), Some(&"car".to_string()));
    assert_eq!(trie.get("care"), Some(&"care".to_string()));
    assert_eq!(trie.get("card"), None);

    // Delete "care" - should not affect the branch point at "car"
    assert_eq!(trie.delete("care"), Some("care".to_string()));
    assert_eq!(trie.len(), 2);
    assert_eq!(trie.get("cat"), Some(&"cat".to_string()));
    assert_eq!(trie.get("car"), Some(&"car".to_string()));
    assert_eq!(trie.get("care"), None);

    // Delete "car" - should not affect "cat"
    assert_eq!(trie.delete("car"), Some("car".to_string()));
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("cat"), Some(&"cat".to_string()));
    assert_eq!(trie.get("car"), None);
}

/// Test that demonstrates the bug if child_remove was changed to child_mut
/// This test specifically targets the cleanup logic
#[test]
fn test_cleanup_bug_detection() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Insert a key that will create intermediate nodes
    trie.insert("test", "test".to_string());

    // Insert another key that shares a prefix but extends further
    trie.insert("testing", "testing".to_string());

    assert_eq!(trie.len(), 2);

    // Delete the longer key - this should trigger cleanup
    assert_eq!(trie.delete("testing"), Some("testing".to_string()));
    assert_eq!(trie.len(), 1);

    // The shorter key should still be accessible
    assert_eq!(trie.get("test"), Some(&"test".to_string()));

    // Now delete the shorter key
    assert_eq!(trie.delete("test"), Some("test".to_string()));
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());

    // Insert a completely different key to verify the trie is clean
    trie.insert("different", "different".to_string());
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("different"), Some(&"different".to_string()));

    // Verify old keys are truly gone
    assert_eq!(trie.get("test"), None);
    assert_eq!(trie.get("testing"), None);
}

/// Test that specifically triggers the cleanup bug by creating a scenario
/// where a single long chain should be completely removed
#[test]
fn test_single_chain_cleanup_bug() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Insert only one key to create a single chain with no branches
    trie.insert("a", "a".to_string());
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("a"), Some(&"a".to_string()));

    // Delete it - this should trigger cleanup of the entire chain
    // If child_mut is used instead of child_remove, the chain won't be cleaned up
    assert_eq!(trie.delete("a"), Some("a".to_string()));
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());
    assert_eq!(trie.get("a"), None);

    // Now insert a different single character key
    // If cleanup didn't work, there might be interference
    trie.insert("b", "b".to_string());
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("b"), Some(&"b".to_string()));
    assert_eq!(trie.get("a"), None);
}

/// Test that creates a scenario where cleanup should definitely be triggered
#[test]
fn test_forced_cleanup_scenario() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Create a scenario where we have a branch point and then a single extension
    trie.insert("ab", "ab".to_string()); // This creates the branch point
    trie.insert("ac", "ac".to_string()); // This creates another branch
    trie.insert("abc", "abc".to_string()); // This extends from the "ab" branch

    assert_eq!(trie.len(), 3);
    assert_eq!(trie.get("ab"), Some(&"ab".to_string()));
    assert_eq!(trie.get("ac"), Some(&"ac".to_string()));
    assert_eq!(trie.get("abc"), Some(&"abc".to_string()));

    // Delete "abc" - this should clean up the "c" extension from "ab"
    // but preserve "ab" since it has a value and "ac" branch exists
    assert_eq!(trie.delete("abc"), Some("abc".to_string()));
    assert_eq!(trie.len(), 2);
    assert_eq!(trie.get("ab"), Some(&"ab".to_string()));
    assert_eq!(trie.get("ac"), Some(&"ac".to_string()));
    assert_eq!(trie.get("abc"), None);

    // Now delete "ab" - this should not affect "ac"
    assert_eq!(trie.delete("ab"), Some("ab".to_string()));
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("ab"), None);
    assert_eq!(trie.get("ac"), Some(&"ac".to_string()));
    assert_eq!(trie.get("abc"), None);

    // Finally delete "ac" - this should leave the trie empty
    assert_eq!(trie.delete("ac"), Some("ac".to_string()));
    assert_eq!(trie.len(), 0);
    assert!(trie.is_empty());
    assert_eq!(trie.get("ab"), None);
    assert_eq!(trie.get("ac"), None);
    assert_eq!(trie.get("abc"), None);
}
