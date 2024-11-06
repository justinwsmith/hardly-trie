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

    #[must_use]
    pub fn get(&self, key: &[u8]) -> Option<&T> {
        let mut current_node = &self.root;
        let mut bytes = key;
        loop {
            if bytes.is_empty() {
                break current_node.value();
            }
            let high_byte: usize = (bytes[0] >> 4).into();
            let low_byte: usize = (bytes[0] & 0x0F).into();

            if current_node.next()[high_byte].is_none() {
                break None;
            }
            current_node = current_node.next()[high_byte].as_ref().unwrap();

            if current_node.next()[low_byte].is_none() {
                break None;
            }
            current_node = current_node.next()[low_byte].as_ref().unwrap();
            bytes = &bytes[1..];
        }
    }


    #[must_use]
    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut T> {
        let mut current_node = &mut self.root;
        let mut bytes = key;
        loop {
            if bytes.is_empty() {
                break current_node.value_mut().as_mut();
            }
            let high_byte: usize = (bytes[0] >> 4).into();
            let low_byte: usize = (bytes[0] & 0x0F).into();

            if current_node.next()[high_byte].is_none() {
                break None;
            }
            current_node = current_node.child_mut(high_byte).as_mut().unwrap();

            if current_node.next()[low_byte].is_none() {
                break None;
            }
            current_node = current_node.child_mut(low_byte).as_mut().unwrap();
            bytes = &bytes[1..];
        }
    }

    #[must_use]
    pub fn delete(&mut self, key: &[u8]) -> Option<T> {
        // TODO: cleanup
        let mut current_node = &mut self.root;
        let mut bytes = key;
        loop {
            if bytes.is_empty() {
                break current_node.value_take();
            }
            let high_byte: usize = (bytes[0] >> 4).into();
            let low_byte: usize = (bytes[0] & 0x0F).into();

            if current_node.next()[high_byte].is_none() {
                break None;
            }
            current_node = current_node.child_mut(high_byte).as_mut().unwrap();

            if current_node.next()[low_byte].is_none() {
                break None;
            }
            current_node = current_node.child_mut(low_byte).as_mut().unwrap();
            bytes = &bytes[1..];
        }
    }


    pub fn insert(&mut self, key: &[u8], mut val: T) -> Option<T> {
        let mut current_node = &mut self.root;
        let mut bytes = key;
        let ret_val = loop {
            if bytes.is_empty() {
                break current_node.value_mut().replace(val);
            }
            let high_byte: usize = (bytes[0] >> 4).into();
            let low_byte: usize = (bytes[0] & 0x0F).into();

            current_node = if current_node.next()[high_byte].is_none() {
                current_node.next_mut()[high_byte].insert(Box::new(TrieNode::new()))
            } else {
                current_node.next_mut()[high_byte].as_mut().unwrap()
            };

            current_node = if current_node.next()[low_byte].is_none() {
                current_node.next_mut()[low_byte].insert(Box::new(TrieNode::new()))
            } else {
                current_node.next_mut()[low_byte].as_mut().unwrap()
            };

            bytes = &bytes[1..];
        };
        if ret_val.is_none() {
            self.len += 1;
        }
        ret_val
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
        let key = "aa".as_bytes();
        assert_eq!(trie.insert(key, 1), None);
        assert_eq!(trie.get(key), Some(&1));


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
        assert_eq!(trie.insert(&[1, 3], 6), Some(5));
    }
}
