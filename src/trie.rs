use crate::trie_node::TrieNode;
use slotmap::{DefaultKey, SlotMap};
use std::marker::PhantomData;

pub struct TrieIter<'a, K, T, const N: usize>
where
    K: TrieKey<N> + ?Sized,
{
    trie: &'a Trie<K, T, N>,
    items: Vec<(Vec<usize>, &'a T)>,
    front_index: usize,
    back_index: usize,
}

impl<'a, K, T, const N: usize> TrieIter<'a, K, T, N>
where
    K: TrieKey<N> + ?Sized,
{
    fn new(trie: &'a Trie<K, T, N>) -> Self {
        let mut items = Vec::new();
        let mut path = Vec::new();
        Self::collect_items(trie, trie.root, &mut path, &mut items);

        let back_index = if items.is_empty() { 0 } else { items.len() - 1 };

        TrieIter {
            trie,
            items,
            front_index: 0,
            back_index,
        }
    }

    fn collect_items(
        trie: &'a Trie<K, T, N>,
        node_key: DefaultKey,
        path: &mut Vec<usize>,
        items: &mut Vec<(Vec<usize>, &'a T)>,
    ) {
        if let Some(node) = trie.arena.get(node_key) {
            // If this node has a value, add it to items
            if let Some(value) = node.value() {
                items.push((path.clone(), value));
            }

            // Recursively visit children in order
            for i in 0..N {
                if let Some(child_key) = node.child_key(i) {
                    path.push(i);
                    Self::collect_items(trie, child_key, path, items);
                    path.pop();
                }
            }
        }
    }
}

impl<'a, K, T, const N: usize> Iterator for TrieIter<'a, K, T, N>
where
    K: TrieKey<N> + ?Sized,
{
    type Item = (Vec<usize>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_index > self.back_index || self.items.is_empty() {
            return None;
        }

        let item = self.items[self.front_index].clone();
        self.front_index += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.front_index > self.back_index || self.items.is_empty() {
            0
        } else {
            self.back_index - self.front_index + 1
        };
        (remaining, Some(remaining))
    }
}

impl<'a, K, T, const N: usize> DoubleEndedIterator for TrieIter<'a, K, T, N>
where
    K: TrieKey<N> + ?Sized,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_index > self.back_index || self.items.is_empty() {
            return None;
        }

        let item = self.items[self.back_index].clone();
        if self.back_index == 0 {
            self.front_index = 1; // Mark as exhausted
        } else {
            self.back_index -= 1;
        }
        Some(item)
    }
}

pub trait TrieKey<const N: usize> {
    fn populate_path(&self, path: &mut Vec<usize>);
    fn init_path(&self) -> Vec<usize>;

    fn build_path(&self) -> Vec<usize> {
        let mut v = self.init_path();
        self.populate_path(&mut v);
        v
    }
}

pub struct Trie<K: TrieKey<N> + ?Sized, T, const N: usize> {
    len: usize,
    arena: SlotMap<DefaultKey, TrieNode<T, N>>,
    root: DefaultKey,
    _key_type: PhantomData<K>,
}

impl<U: AsRef<[u8]> + ?Sized> TrieKey<16> for U {
    fn populate_path(&self, path: &mut Vec<usize>) {
        for &byte in self.as_ref() {
            let high_byte: usize = (byte >> 4).into();
            let low_byte: usize = (byte & 0x0F).into();
            path.push(high_byte);
            path.push(low_byte);
        }
    }

    fn init_path(&self) -> Vec<usize> {
        Vec::with_capacity(2 * self.as_ref().len())
    }
}

impl<K: TrieKey<N> + ?Sized, T, const N: usize> Trie<K, T, N> {
    #[must_use]
    pub fn new() -> Trie<K, T, N> {
        let mut arena = SlotMap::new();
        let root = arena.insert(TrieNode::new());
        Trie {
            len: 0,
            arena,
            root,
            _key_type: PhantomData,
        }
    }

    #[must_use]
    pub fn get(&self, key: &K) -> Option<&T> {
        let mut current_key = self.root;
        let path = key.build_path();
        for child_index in path {
            let current_node = self.arena.get(current_key)?;
            if let Some(child_key) = current_node.child_key(child_index) {
                current_key = child_key;
            } else {
                return None;
            }
        }
        self.arena.get(current_key)?.value()
    }

    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut T> {
        let mut current_key = self.root;
        let path = key.build_path();
        for child_index in path {
            let child_key = {
                let current_node = self.arena.get(current_key)?;
                current_node.child_key(child_index)?
            };
            current_key = child_key;
        }
        self.arena.get_mut(current_key)?.value_mut()
    }

    #[must_use]
    pub fn delete(&mut self, key: &K) -> Option<T> {
        let path = key.build_path();
        let mut node_path = Vec::with_capacity(path.len() + 1);
        let mut current_key = self.root;
        node_path.push(current_key);

        // Navigate to the target node, building the path
        for &child_index in &path {
            let child_key = {
                let current_node = self.arena.get(current_key)?;
                current_node.child_key(child_index)?
            };
            current_key = child_key;
            node_path.push(current_key);
        }

        // Check if the target node has a value to delete
        let target_node = self.arena.get(current_key)?;
        if target_node.value().is_none() {
            return None;
        }

        // Find the cleanup point BEFORE removing the value
        let mut cleanup_index = None;
        for (i, &node_key) in node_path.iter().enumerate().rev() {
            let node = self.arena.get(node_key)?;
            let has_value = if i == node_path.len() - 1 {
                // For the target node, we're about to remove its value, so check if it has children
                node.has_child()
            } else {
                // For other nodes, check if they have a value or multiple children
                node.value().is_some() || node.has_multiple_children()
            };

            if has_value {
                cleanup_index = Some(i);
                break;
            }
            if i == 0 {
                // We're at the root
                cleanup_index = Some(0);
                break;
            }
        }

        // Take the value from the target node
        let retval = self.arena.get_mut(current_key)?.value_take();
        self.len -= 1;

        // Perform cleanup if needed
        if let Some(cleanup_idx) = cleanup_index {
            if cleanup_idx < path.len() {
                let parent_key = node_path[cleanup_idx];
                let child_index = path[cleanup_idx];
                let child_key = self.arena.get(parent_key)?.child_key(child_index)?;

                // Remove the child reference
                self.arena.get_mut(parent_key)?.child_remove(child_index);

                // Remove all nodes from the arena that are no longer reachable
                self.cleanup_unreachable_nodes(child_key);
            }
        }

        retval
    }

    fn cleanup_unreachable_nodes(&mut self, start_key: DefaultKey) {
        let mut to_remove = Vec::new();
        let mut stack = vec![start_key];

        while let Some(key) = stack.pop() {
            if let Some(node) = self.arena.get(key) {
                // Add all children to the stack
                for i in 0..N {
                    if let Some(child_key) = node.child_key(i) {
                        stack.push(child_key);
                    }
                }
                to_remove.push(key);
            }
        }

        // Remove all collected nodes
        for key in to_remove {
            self.arena.remove(key);
        }
    }

    pub fn insert(&mut self, key: &K, val: T) -> Option<T> {
        let mut current_key = self.root;
        let path = key.build_path();

        for child_index in path {
            let child_key = {
                let current_node = self.arena.get(current_key).unwrap();
                current_node.child_key(child_index)
            };

            if let Some(existing_child_key) = child_key {
                current_key = existing_child_key;
            } else {
                // Create new node
                let new_node = TrieNode::new();
                let new_key = self.arena.insert(new_node);
                self.arena
                    .get_mut(current_key)
                    .unwrap()
                    .child_set(child_index, new_key);
                current_key = new_key;
            }
        }

        let current_node = self.arena.get_mut(current_key).unwrap();
        if current_node.value().is_none() {
            self.len += 1;
        }
        current_node.value_replace(val)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an iterator over the trie's key-value pairs.
    /// The iterator yields `(Vec<usize>, &T)` where the `Vec<usize>` represents
    /// the path indices that make up the key.
    pub fn iter(&self) -> TrieIter<K, T, N> {
        TrieIter::new(self)
    }
}

impl<K, T, const N: usize> Default for Trie<K, T, N>
where
    K: TrieKey<N>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::{Trie, TrieKey};

    #[test]
    fn it_works() {
        let mut trie: Trie<[u8], usize, 16> = Trie::new();

        assert_eq!(trie.insert(&[1, 3, 7, 2], 3), None);
        assert_eq!(trie.get(&[]), None);
        assert_eq!(trie.get(&[1]), None);
        assert_eq!(trie.get(&[1, 3]), None);
        assert_eq!(trie.get(&[1, 3, 7]), None);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
        assert_eq!(trie.insert(&[], 7), None);
        assert_eq!(trie.get(&[]), Some(&7));
        assert_eq!(trie.get(&[1]), None);
        assert_eq!(trie.get(&[1, 3]), None);
        assert_eq!(trie.get(&[1, 3, 7]), None);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
        assert_eq!(trie.insert(&[1, 3], 5), None);
        assert_eq!(trie.get(&[]), Some(&7));
        assert_eq!(trie.get(&[1]), None);
        assert_eq!(trie.get(&[1, 3]), Some(&5));
        assert_eq!(trie.get(&[1, 3, 7]), None);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
        assert_eq!(trie.len(), 3);
        assert_eq!(trie.insert(&[1, 3], 6), Some(5));
        assert_eq!(trie.len(), 3);
        assert_eq!(trie.delete(&[1, 3]), Some(6));
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
        assert_eq!(trie.delete(&[1, 3, 7, 2]), Some(3));
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.delete(&[]), Some(7));
        assert_eq!(trie.len(), 0);
    }

    /// Test that verifies internal node cleanup by directly inspecting trie structure.
    /// This test would fail if child_remove was changed to child_mut in the delete function.
    #[test]
    fn test_internal_node_cleanup() {
        let mut trie: Trie<str, String, 16> = Trie::new();

        // Insert "test" and "testing" - "testing" extends "test" with "ing"
        trie.insert("test", "test_value".to_string());
        trie.insert("testing", "testing_value".to_string());

        assert_eq!(trie.len(), 2);
        assert_eq!(trie.get("test"), Some(&"test_value".to_string()));
        assert_eq!(trie.get("testing"), Some(&"testing_value".to_string()));

        // Navigate to the "test" node and verify it has children (for "ing")
        let mut current_key = trie.root;
        let test_path = "test".build_path();
        for &child_index in &test_path {
            let current_node = trie.arena.get(current_key).unwrap();
            current_key = current_node.child_key(child_index).unwrap();
        }
        // At this point, current_node should have children for the "ing" extension
        let current_node = trie.arena.get(current_key).unwrap();
        assert!(
            current_node.has_child(),
            "Node for 'test' should have children for 'ing' extension"
        );

        // Delete "testing" - this should clean up the "ing" part
        assert_eq!(trie.delete("testing"), Some("testing_value".to_string()));
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.get("test"), Some(&"test_value".to_string()));
        assert_eq!(trie.get("testing"), None);

        // Navigate to the "test" node again and verify it no longer has children
        let mut current_key = trie.root;
        for &child_index in &test_path {
            let current_node = trie.arena.get(current_key).unwrap();
            current_key = current_node.child_key(child_index).unwrap();
        }
        // If cleanup worked properly, the "test" node should no longer have children
        let current_node = trie.arena.get(current_key).unwrap();
        assert!(
            !current_node.has_child(),
            "Node for 'test' should not have children after 'testing' is deleted"
        );
    }

    /// Test cleanup of a single long chain with no branches
    #[test]
    fn test_single_chain_internal_cleanup() {
        let mut trie: Trie<str, String, 16> = Trie::new();

        // Insert a single key to create a chain
        trie.insert("a", "value_a".to_string());
        assert_eq!(trie.len(), 1);

        // Verify the root has a child
        let root_node = trie.arena.get(trie.root).unwrap();
        assert!(root_node.has_child(), "Root should have a child for 'a'");

        // Delete the key - this should clean up the entire chain
        assert_eq!(trie.delete("a"), Some("value_a".to_string()));
        assert_eq!(trie.len(), 0);

        // Verify the root no longer has any children
        let root_node = trie.arena.get(trie.root).unwrap();
        assert!(
            !root_node.has_child(),
            "Root should not have any children after deleting 'a'"
        );
    }

    /// Test that intermediate nodes with values are preserved during cleanup
    #[test]
    fn test_preserve_intermediate_nodes_internal() {
        let mut trie: Trie<str, String, 16> = Trie::new();

        // Create a scenario: "app", "apple"
        trie.insert("app", "app_value".to_string());
        trie.insert("apple", "apple_value".to_string());

        assert_eq!(trie.len(), 2);

        // Navigate to the "app" node and verify it has children for "le"
        let mut current_key = trie.root;
        let app_path = "app".build_path();
        for &child_index in &app_path {
            let current_node = trie.arena.get(current_key).unwrap();
            current_key = current_node.child_key(child_index).unwrap();
        }
        let current_node = trie.arena.get(current_key).unwrap();
        assert!(
            current_node.has_child(),
            "Node for 'app' should have children for 'le' extension"
        );
        assert!(
            current_node.value().is_some(),
            "Node for 'app' should have a value"
        );

        // Delete "apple" - should not affect "app" node since it has a value
        assert_eq!(trie.delete("apple"), Some("apple_value".to_string()));
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.get("app"), Some(&"app_value".to_string()));

        // Navigate to the "app" node again - it should still exist but have no children
        let mut current_key = trie.root;
        for &child_index in &app_path {
            let current_node = trie.arena.get(current_key).unwrap();
            current_key = current_node.child_key(child_index).unwrap();
        }
        let current_node = trie.arena.get(current_key).unwrap();
        assert!(
            !current_node.has_child(),
            "Node for 'app' should not have children after 'apple' is deleted"
        );
        assert!(
            current_node.value().is_some(),
            "Node for 'app' should still have its value"
        );
    }

    /// Test the specific bug scenario - this test will fail if child_remove is changed to child_mut
    #[test]
    fn test_cleanup_bug_detection_internal() {
        let mut trie: Trie<str, String, 16> = Trie::new();

        // Insert "ab" and "abc" where "abc" extends "ab"
        trie.insert("ab", "ab_value".to_string());
        trie.insert("abc", "abc_value".to_string());

        assert_eq!(trie.len(), 2);

        // Navigate to the "ab" node and verify it has a child for "c"
        let mut current_key = trie.root;
        let ab_path = "ab".build_path();
        for &child_index in &ab_path {
            let current_node = trie.arena.get(current_key).unwrap();
            current_key = current_node.child_key(child_index).unwrap();
        }
        let current_node = trie.arena.get(current_key).unwrap();
        assert!(
            current_node.has_child(),
            "Node for 'ab' should have a child for 'c'"
        );

        // Delete "abc" - this should trigger cleanup of the "c" extension
        assert_eq!(trie.delete("abc"), Some("abc_value".to_string()));
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.get("ab"), Some(&"ab_value".to_string()));
        assert_eq!(trie.get("abc"), None);

        // Navigate to the "ab" node again - if cleanup worked, it should have no children
        // If the bug exists (child_mut instead of child_remove), the "c" node will still be there
        let mut current_key = trie.root;
        for &child_index in &ab_path {
            let current_node = trie.arena.get(current_key).unwrap();
            current_key = current_node.child_key(child_index).unwrap();
        }
        let current_node = trie.arena.get(current_key).unwrap();
        assert!(
            !current_node.has_child(),
            "CLEANUP BUG: Node for 'ab' should not have children after 'abc' is deleted. \
             If this fails, check that delete() uses child_remove() not child_mut()"
        );
    }

    #[test]
    fn test_iterator_empty_trie() {
        let trie: Trie<str, String, 16> = Trie::new();
        let mut iter = trie.iter();

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_iterator_single_element() {
        let mut trie: Trie<str, String, 16> = Trie::new();
        trie.insert("a", "value_a".to_string());

        let mut iter = trie.iter();
        assert_eq!(iter.size_hint(), (1, Some(1)));

        // Test forward iteration
        let (path, value) = iter.next().unwrap();
        assert_eq!(value, &"value_a".to_string());
        assert_eq!(path, "a".build_path());
        assert_eq!(iter.next(), None);

        // Test backward iteration on fresh iterator
        let mut iter = trie.iter();
        let (path, value) = iter.next_back().unwrap();
        assert_eq!(value, &"value_a".to_string());
        assert_eq!(path, "a".build_path());
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_iterator_multiple_elements() {
        let mut trie: Trie<str, String, 16> = Trie::new();
        trie.insert("a", "value_a".to_string());
        trie.insert("b", "value_b".to_string());
        trie.insert("ab", "value_ab".to_string());

        // Collect all elements via forward iteration
        let forward_items: Vec<_> = trie.iter().collect();
        assert_eq!(forward_items.len(), 3);

        // Collect all elements via backward iteration
        let backward_items: Vec<_> = trie.iter().rev().collect();
        assert_eq!(backward_items.len(), 3);

        // Forward and backward should be reverses of each other
        let mut backward_reversed = backward_items;
        backward_reversed.reverse();
        assert_eq!(forward_items, backward_reversed);
    }

    #[test]
    fn test_iterator_bidirectional() {
        let mut trie: Trie<str, String, 16> = Trie::new();
        trie.insert("a", "value_a".to_string());
        trie.insert("b", "value_b".to_string());
        trie.insert("c", "value_c".to_string());
        trie.insert("d", "value_d".to_string());

        let mut iter = trie.iter();
        assert_eq!(iter.size_hint(), (4, Some(4)));

        // Take one from front
        let (_, front_val) = iter.next().unwrap();
        assert_eq!(iter.size_hint(), (3, Some(3)));

        // Take one from back
        let (_, back_val) = iter.next_back().unwrap();
        assert_eq!(iter.size_hint(), (2, Some(2)));

        // Take another from front
        let (_, front_val2) = iter.next().unwrap();
        assert_eq!(iter.size_hint(), (1, Some(1)));

        // Take last from back
        let (_, back_val2) = iter.next_back().unwrap();
        assert_eq!(iter.size_hint(), (0, Some(0)));

        // Should be exhausted
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        // Verify we got different values
        let all_values = vec![front_val, front_val2, back_val2, back_val];
        let unique_values: std::collections::HashSet<_> = all_values.iter().collect();
        assert_eq!(unique_values.len(), 4);
    }

    #[test]
    fn test_iterator_path_reconstruction() {
        let mut trie: Trie<[u8], String, 16> = Trie::new();

        // Insert some byte arrays
        trie.insert(&[0x12], "value_12".to_string());
        trie.insert(&[0x34, 0x56], "value_3456".to_string());
        trie.insert(&[], "value_empty".to_string());

        let items: Vec<_> = trie.iter().collect();
        assert_eq!(items.len(), 3);

        // Verify paths match the original keys
        for (path, value) in items {
            if value == &"value_12".to_string() {
                assert_eq!(path, [0x12u8].build_path());
            } else if value == &"value_3456".to_string() {
                assert_eq!(path, [0x34u8, 0x56u8].build_path());
            } else if value == &"value_empty".to_string() {
                assert_eq!(path, [].build_path());
            } else {
                panic!("Unexpected value: {}", value);
            }
        }
    }

    #[test]
    fn test_iterator_lexicographic_order() {
        let mut trie: Trie<str, String, 16> = Trie::new();

        // Insert in non-lexicographic order
        trie.insert("zebra", "zebra".to_string());
        trie.insert("apple", "apple".to_string());
        trie.insert("banana", "banana".to_string());
        trie.insert("cherry", "cherry".to_string());

        // Forward iteration should be in lexicographic order
        let forward_values: Vec<_> = trie.iter().map(|(_, v)| v.clone()).collect();
        let mut expected = forward_values.clone();
        expected.sort();
        assert_eq!(forward_values, expected);

        // Backward iteration should be in reverse lexicographic order
        let backward_values: Vec<_> = trie.iter().rev().map(|(_, v)| v.clone()).collect();
        let mut expected_rev = expected;
        expected_rev.reverse();
        assert_eq!(backward_values, expected_rev);
    }

    #[test]
    fn test_iterator_with_common_prefixes() {
        let mut trie: Trie<str, String, 16> = Trie::new();

        // Insert words with common prefixes
        trie.insert("test", "test".to_string());
        trie.insert("testing", "testing".to_string());
        trie.insert("tester", "tester".to_string());
        trie.insert("tea", "tea".to_string());
        trie.insert("team", "team".to_string());

        let all_items: Vec<_> = trie.iter().map(|(_, v)| v.clone()).collect();
        assert_eq!(all_items.len(), 5);

        // Should be in lexicographic order
        let mut expected = all_items.clone();
        expected.sort();
        assert_eq!(all_items, expected);

        // Test that we can iterate both ways and get all items
        let forward_count = trie.iter().count();
        let backward_count = trie.iter().rev().count();
        assert_eq!(forward_count, 5);
        assert_eq!(backward_count, 5);
    }
}
