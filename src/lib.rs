#![allow(unused)]
#![allow(clippy::missing_panics_doc)]

use std::collections::LinkedList;

struct TrieNode<T> {
    value: Option<T>,
    next: [Option<Box<TrieNode<T>>>; 16],
}

impl<T> TrieNode<T> {
    #[must_use]
    fn new() -> TrieNode<T> {
        TrieNode {
            value: const { None },
            next: [const { None }; 16],
        }
    }

    fn count_children(&self) -> usize {
        let mut count = 0;
        for i in 0..16 {
            if self.next[i].is_some() {
                count += 1;
            }
        }
        count
    }

    fn value_take(&mut self) -> Option<T> {
        self.value.take()
    }

    fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    fn value_mut(&mut self) -> &mut Option<T> {
        &mut self.value
    }

    fn next(&self) -> &[Option<Box<TrieNode<T>>>; 16] {
        &self.next
    }

    fn next_mut(&mut self) -> &mut [Option<Box<TrieNode<T>>>; 16] {
        &mut self.next
    }

    fn child(&self, index: usize) -> Option<&TrieNode<T>> {
        self.next[index].as_deref()
    }

    fn child_mut(&mut self, index: usize) -> &mut Option<Box<TrieNode<T>>> {
        &mut self.next[index]
    }
}

pub struct Trie<T> {
    len: usize,
    root: TrieNode<T>,
}

impl<T> Trie<T> {
    #[must_use]
    pub fn new() -> Trie<T> {
        Trie {
            len: 0,
            root: TrieNode::new(),
        }
    }

    fn build_path(bytes: &[u8], path: &mut Vec<usize>) {
        for &byte in bytes {
            let high_byte: usize = (byte >> 4).into();
            let low_byte: usize = (byte & 0x0F).into();
            path.push(high_byte);
            path.push(low_byte);
        }
    }

    #[must_use]
    pub fn get(&self, key: &[u8]) -> Option<&T> {
        let mut current_node = &self.root;
        let mut path = Vec::with_capacity(2 * key.len());
        Self::build_path(key, &mut path);
        for child_index in path {
            if let Some(node) = current_node.child(child_index) {
                current_node = node;
            } else {
                return None;
            }
        }
        current_node.value()
    }


    #[must_use]
    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut T> {
        let mut current_node = &mut self.root;
        let mut path = Vec::with_capacity(2 * key.len());
        Self::build_path(key, &mut path);
        for child_index in path {
            if let Some(node) = current_node.child_mut(child_index) {
                current_node = node;
            } else {
                return None;
            }
        }
        current_node.value_mut().as_mut()
    }

    #[must_use]
    pub fn delete(&mut self, key: &[u8]) -> Option<T> {
        let mut current_node = &mut self.root;
        let mut path = Vec::with_capacity(2 * key.len() + 1);
        Self::build_path(key, &mut path);
        let mut branch_base = None;
        for (i, &child_index) in path.iter().enumerate() {
            if current_node.value().is_some() || current_node.count_children() > 1 || branch_base.is_none() {
                branch_base = Some(i);
            }
            if let Some(node) = current_node.child_mut(child_index).as_mut() {
                current_node = node;
            } else {
                return None;
            }
        }
        if current_node.count_children() > 0 {
            branch_base = None;
        }
        let retval = current_node.value.take();

        // Cleanup
        if retval.is_some() {
            if let Some(path_index) = branch_base {
                current_node = &mut self.root;
                for &child_index in path.iter().take(path_index) {
                    current_node = current_node.child_mut(child_index).as_mut().unwrap();
                }
                current_node.child_mut(path[path_index]).take();
            }
            self.len -= 1;
        }
        retval
    }

    pub fn insert(&mut self, key: &[u8], mut val: T) -> Option<T> {
        let mut current_node = &mut self.root;
        let mut path = Vec::with_capacity(2 * key.len());
        Self::build_path(key, &mut path);
        for child_index in path {
            current_node = current_node.child_mut(child_index).get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        if current_node.value().is_none() {
            self.len += 1;
        }
        current_node.value_mut().replace(val)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie: Trie<usize> = Trie::new();

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
}
