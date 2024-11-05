#![allow(unused)]

enum TrieNode<T> {
    LowNode { value: Option<T>, next: [Option<Box<TrieNode<T>>>; 16] },
    HighNode { next: [Option<Box<TrieNode<T>>>; 16] },
}

impl<T> TrieNode<T> {
    #[must_use]
    fn new_low_node() -> TrieNode<T> {
        TrieNode::LowNode {
            value: const { None },
            next: [const { None }; 16],
        }
    }

    #[must_use]
    fn new_high_node() -> TrieNode<T> {
        TrieNode::HighNode {
            next: [const { None }; 16],
        }
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
            root: TrieNode::new_low_node(),
        }
    }

    #[must_use]
    pub fn get(&self, key: &[u8]) -> Option<&T> {
        let mut current_node = &self.root;
        let mut bytes = key;
        loop {
            match (current_node) {
                TrieNode::LowNode { value, next } => {
                    if bytes.is_empty() {
                        return value.as_ref();
                    } else {
                        let byte = bytes[0];
                        let high_byte: usize = (byte >> 4).into();
                        if let Some(high_node) = &next[high_byte] {
                            current_node = high_node;
                        } else {
                            return None;
                        }
                    }
                }
                TrieNode::HighNode { next } => {
                    let byte = bytes[0];
                    let low_byte: usize = (byte & 0x0F).into();
                    if let Some(low_node) = &next[low_byte] {
                        bytes = &bytes[1..];
                        current_node = low_node;
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, key: &[u8], val: T) -> Option<T> {
        let mut current_node = &mut self.root;
        let mut bytes = key;
        let ret_val = loop {
            match (current_node) {
                TrieNode::LowNode { value, next } => {
                    if bytes.is_empty() {
                        break value.replace(val);
                    } else {
                        let byte = bytes[0];
                        let high_byte: usize = (byte >> 4).into();
                        if next[high_byte].is_none() {
                            next[high_byte].replace(Box::new(TrieNode::new_high_node()));
                        }
                        current_node = next[high_byte].as_mut().unwrap();
                    }
                }
                TrieNode::HighNode { next } => {
                    let byte = bytes[0];
                    let low_byte: usize = (byte & 0x0F).into();
                    bytes = &bytes[1..];
                    if next[low_byte].is_none() {
                        next[low_byte].replace(Box::new(TrieNode::new_low_node()));
                    }
                    current_node = next[low_byte].as_mut().unwrap();
                }
            }
        };
        if ret_val.is_none() {
            self.len += 1;
        }
        ret_val
    }

    pub fn len(&self) -> usize {
        self.len
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
