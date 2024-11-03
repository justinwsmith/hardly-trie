#![allow(unused)]


struct LowNode<T> {
    value: Option<T>,
    next: [Option<Box<HighNode<T>>>; 16],
}

impl<T> LowNode<T> {
    #[must_use]
    fn new() -> LowNode<T> {
        LowNode {
            value: const { None },
            next: [const { None }; 16],
        }
    }
    #[must_use]
    fn get(&self, bytes: &[u8]) -> Option<&T> {
        let next_bytes = &bytes[1..];
        if next_bytes.is_empty() {
            self.value.as_ref()
        } else {
            let byte = next_bytes[0];
            let high_byte: usize = (byte >> 4).into();
            if let Some(high_node) = &self.next[high_byte] {
                high_node.get(next_bytes)
            } else {
                None
            }
        }
    }
    #[must_use]
    fn get_mut(&mut self, bytes: &[u8]) -> Option<&mut T> {
        let next_bytes = &bytes[1..];
        if next_bytes.is_empty() {
            self.value.as_mut()
        } else {
            let byte = next_bytes[0];
            let high_byte: usize = (byte >> 4).into();
            if let Some(high_node) = &mut self.next[high_byte] {
                high_node.get_mut(next_bytes)
            } else {
                None
            }
        }
    }
    fn insert(&mut self, bytes: &[u8], val: T) -> Option<T> {
        let next_bytes = &bytes[1..];
        if next_bytes.is_empty() {
            self.value.replace(val)
        } else {
            let byte = next_bytes[0];
            let high_byte: usize = (byte >> 4).into();
            if let Some(high_node) = &mut self.next[high_byte] {
                high_node.insert(next_bytes, val)
            } else {
                let mut high_node = Box::new(HighNode::new());
                high_node.insert(next_bytes, val);
                self.next[high_byte].replace(high_node);
                None
            }
        }
    }
}

struct HighNode<T> {
    next: [Option<Box<LowNode<T>>>; 16],
}

impl<T> HighNode<T> {
    #[must_use]
    fn new() -> HighNode<T> {
        HighNode {
            next: [const { None }; 16],
        }
    }
    #[must_use]
    fn get(&self, bytes: &[u8]) -> Option<&T> {
        let byte = bytes[0];
        let low_byte: usize = (byte & 0x0F).into();
        if let Some(low_node) = &self.next[low_byte] {
            low_node.get(bytes)
        } else {
            None
        }
    }
    #[must_use]
    fn get_mut(&mut self, bytes: &[u8]) -> Option<&mut T> {
        let byte = bytes[0];
        let low_byte: usize = (byte & 0x0F).into();
        if let Some(low_node) = &mut self.next[low_byte] {
            low_node.get_mut(bytes)
        } else {
            None
        }
    }
    fn insert(&mut self, bytes: &[u8], val: T) -> Option<T> {
        let byte = bytes[0];
        let low_byte: usize = (byte & 0x0F).into();
        if let Some(low_node) = &mut self.next[low_byte] {
            low_node.insert(bytes, val)
        } else {
            let mut low_node = Box::new(LowNode::new());
            low_node.insert(bytes, val);
            self.next[low_byte].replace(low_node);
            None
        }
    }
}

pub struct Trie<T> {
    len: usize,
    value: Option<T>,
    next: [Option<Box<HighNode<T>>>; 16],
}

impl<T> Trie<T> {
    #[must_use]
    pub fn new() -> Trie<T> {
        Trie {
            len: 0,
            value: const { None },
            next: [const { None }; 16],
        }
    }
    #[must_use]
    pub fn get(&self, bytes: &[u8]) -> Option<&T> {
        if bytes.is_empty() {
            self.value.as_ref()
        } else {
            let byte = bytes[0];
            let high_byte: usize = (byte >> 4).into();
            if let Some(high_node) = &self.next[high_byte] {
                high_node.get(bytes)
            } else {
                None
            }
        }
    }
    #[must_use]
    pub fn get_mut(&mut self, bytes: &[u8]) -> Option<&mut T> {
        if bytes.is_empty() {
            self.value.as_mut()
        } else {
            let byte = bytes[0];
            let high_byte: usize = (byte >> 4).into();
            if let Some(high_node) = &mut self.next[high_byte] {
                high_node.get_mut(bytes)
            } else {
                None
            }
        }
    }
    pub fn insert(&mut self, bytes: &[u8], val: T) -> Option<T> {
        let retval = if bytes.is_empty() {
            self.value.replace(val)
        } else {
            let byte = bytes[0];
            let high_byte: usize = (byte >> 4).into();
            if let Some(high_node) = &mut self.next[high_byte] {
                high_node.insert(bytes, val)
            } else {
                let mut high_node = Box::new(HighNode::new());
                high_node.insert(bytes, val);
                self.next[high_byte].replace(high_node);
                None
            }
        };
        if retval.is_none() { self.len += 1; }
        retval
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
        trie.insert(key, 1);
        assert_eq!(trie.get(key), Some(&1));


        trie.insert(&[1, 3, 7, 2], 3);
        assert_eq!(trie.get(&[]), None);
        assert_eq!(trie.get(&[1]), None);
        assert_eq!(trie.get(&[1, 3]), None);
        assert_eq!(trie.get(&[1, 3, 7]), None);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
        trie.insert(&[], 7);
        assert_eq!(trie.get(&[]), Some(&7));
        assert_eq!(trie.get(&[1]), None);
        assert_eq!(trie.get(&[1, 3]), None);
        assert_eq!(trie.get(&[1, 3, 7]), None);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
        trie.insert(&[1, 3], 5);
        assert_eq!(trie.get(&[]), Some(&7));
        assert_eq!(trie.get(&[1]), None);
        assert_eq!(trie.get(&[1, 3]), Some(&5));
        assert_eq!(trie.get(&[1, 3, 7]), None);
        assert_eq!(trie.get(&[1, 3, 7, 2]), Some(&3));
    }
}
