use crate::trie_node::{TrieNode, TrieNodeChildIterator};
use std::marker::PhantomData;

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
    root: TrieNode<T, N>,
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
        Trie {
            len: 0,
            root: TrieNode::new(),
            _key_type: PhantomData,
        }
    }

    #[must_use]
    pub fn get(&self, key: &K) -> Option<&T> {
        let mut current_node = &self.root;
        let mut path = key.build_path();
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
    pub fn get_mut(&mut self, key: &K) -> Option<&mut T> {
        let mut current_node = &mut self.root;
        let mut path = key.build_path();
        for child_index in path {
            if let Some(node) = current_node.child_mut(child_index) {
                current_node = node;
            } else {
                return None;
            }
        }
        current_node.value_mut()
    }

    #[must_use]
    pub fn delete(&mut self, key: &K) -> Option<T> {
        let mut current_node = &mut self.root;
        let mut path = key.build_path();
        let mut branch_base = None;
        for (i, &child_index) in path.iter().enumerate() {
            if current_node.value().is_some()
                || current_node.has_multiple_children()
                || branch_base.is_none()
            {
                branch_base = Some(i);
            }
            if let Some(node) = current_node.child_mut(child_index) {
                current_node = node;
            } else {
                return None;
            }
        }
        if current_node.has_child() {
            branch_base = None;
        }
        let retval = current_node.value_take();

        // Cleanup
        if retval.is_some() {
            if let Some(path_index) = branch_base {
                current_node = &mut self.root;
                for &child_index in path.iter().take(path_index) {
                    current_node = current_node.child_mut(child_index).unwrap();
                }
                current_node.child_mut(path[path_index]);
            }
            self.len -= 1;
        }
        retval
    }

    pub fn insert(&mut self, key: &K, val: T) -> Option<T> {
        let mut current_node = &mut self.root;
        let mut path = key.build_path();
        for child_index in path {
            if current_node.child(child_index).is_some() {
                current_node = current_node.child_mut(child_index).unwrap();
            } else {
                current_node = current_node.child_set(child_index, TrieNode::new());
            }
        }
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
    use crate::trie::Trie;

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
}
