#[cfg(feature = "bitmaps")]
use bitmaps::Bitmap;
#[cfg(feature = "bitmaps")]
const BITMAP_SIZE: usize = 64;

pub(crate) struct TrieNode<T, const N: usize> {
    #[cfg(feature = "bitmaps")]
    child_bits: Bitmap<BITMAP_SIZE>,
    value: Option<T>,
    next: [Option<Box<TrieNode<T, N>>>; N],
}

impl<T, const N: usize> TrieNode<T, N> {
    #[must_use]
    pub(crate) fn new() -> TrieNode<T, N> {
        TrieNode {
            #[cfg(feature = "bitmaps")]
            child_bits: Bitmap::new(),
            value: const { None },
            next: [const { None }; N],
        }
    }

    pub(crate) fn has_child(&self) -> bool {
        #[cfg(feature = "bitmaps")]
        if const { N <= BITMAP_SIZE } {
            return !self.child_bits.is_empty();
        }
        for i in 0..N {
            if self.child(i).is_some() {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_multiple_children(&self) -> bool {
        #[cfg(feature = "bitmaps")]
        if const { N <= BITMAP_SIZE } {
            return self.child_bits.len() > 1;
        }
        let mut count = 0;
        for i in 0..N {
            if self.child(i).is_some() {
                if count > 0 {
                    return true;
                }
                count += 1;
            }
        }
        false
    }

    pub(crate) fn value_take(&mut self) -> Option<T> {
        self.value.take()
    }

    pub(crate) fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub(crate) fn value_replace(&mut self, val: T) -> Option<T> {
        self.value.replace(val)
    }

    pub(crate) fn value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    pub(crate) fn child(&self, index: usize) -> Option<&TrieNode<T, N>> {
        self.next[index].as_deref()
    }

    pub(crate) fn child_mut(&mut self, index: usize) -> Option<&mut TrieNode<T, N>> {
        self.next[index].as_deref_mut()
    }

    pub(crate) fn child_set(&mut self, index: usize, node: TrieNode<T, N>) -> &mut TrieNode<T, N> {
        #[cfg(feature = "bitmaps")]
        if const { N <= BITMAP_SIZE } {
            self.child_bits.set(index, true);
        }
        self.next[index].insert(Box::new(node))
    }

    pub(crate) fn child_iter_from(&self, index: usize) -> TrieNodeChildIterator<'_, T, N> {
        TrieNodeChildIterator {
            moved: false,
            index,
            node: self,
        }
    }
}

pub(crate) struct TrieNodeChildIterator<'a, T, const N: usize> {
    moved: bool,
    index: usize,
    node: &'a TrieNode<T, N>,
}

impl<'a, T, const N: usize> Iterator for TrieNodeChildIterator<'a, T, N> {
    type Item = &'a TrieNode<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "bitmaps")]
        if const { N <= BITMAP_SIZE } {
            let my_index_opt = if !self.moved && self.index == 0 {
                self.node.child_bits.first_index()
            } else {
                self.node.child_bits.next_index(self.index)
            };
            return if let Some(index) = my_index_opt {
                self.moved = true;
                self.index = index;
                self.node.child(self.index)
            } else {
                None
            };
        }
        let mut i = self.index;
        if self.moved || self.index != 0 {
            i += 1;
        }
        while i < N {
            let child_opt = self.node.child(i);
            if let Some(child) = child_opt {
                self.moved = true;
                self.index = i;
                return Some(child);
            }
            i += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        #[cfg(feature = "bitmaps")]
        return (0, Some(self.node.child_bits.len()));
        #[cfg(not(feature = "bitmaps"))]
        (0, Some(N))
    }
}

impl<T, const N: usize> DoubleEndedIterator for TrieNodeChildIterator<'_, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "bitmaps")]
        if const { N <= BITMAP_SIZE } {
            let my_index_opt = if !self.moved && self.index == 0 {
                self.node.child_bits.last_index()
            } else {
                self.node.child_bits.prev_index(self.index)
            };
            return if let Some(index) = my_index_opt {
                self.moved = true;
                self.index = index;
                self.node.child(self.index)
            } else {
                None
            };
        }
        let mut i = if !self.moved && self.index == 0 {
            N - 1
        } else {
            self.index - 1
        };
        while i > 0 {
            let child_opt = self.node.child(i);
            if let Some(child) = child_opt {
                self.moved = true;
                self.index = i;
                return Some(child);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        None
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a TrieNode<T, N> {
    type Item = &'a TrieNode<T, N>;
    type IntoIter = TrieNodeChildIterator<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        TrieNodeChildIterator {
            moved: false,
            index: 0,
            node: self,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::trie_node::TrieNode;

    #[test]
    fn test_iterator_forward() {
        let mut root = TrieNode::<usize, 16>::new();
        let mut child2 = TrieNode::<usize, 16>::new();
        child2.value_replace(2);
        let mut child13 = TrieNode::<usize, 16>::new();
        child13.value_replace(13);
        root.child_set(2, child2);
        root.child_set(13, child13);

        let mut i: usize = 0;
        for node in &root {
            if i == 0 {
                assert_eq!(node.value(), Some(&2));
            } else if i == 1 {
                assert_eq!(node.value(), Some(&13));
            } else {
                panic!("Only 2 expected");
            }
            i += 1;
        }
        assert_eq!(i, 2);

        let mut iter = &mut root.into_iter();
        assert_eq!(iter.next().unwrap().value(), Some(&2));
        assert!(iter.next_back().is_none());

        let mut iter = &mut root.into_iter();
        assert_eq!(iter.next().unwrap().value(), Some(&2));
        assert_eq!(iter.next().unwrap().value(), Some(&13));
        assert_eq!(iter.next_back().unwrap().value(), Some(&2));
        assert!(iter.next_back().is_none());

        let mut iter = &mut root.into_iter();
        assert_eq!(iter.next().unwrap().value(), Some(&2));
        assert_eq!(iter.next().unwrap().value(), Some(&13));
        assert_eq!(iter.next_back().unwrap().value(), Some(&2));
        assert_eq!(iter.next().unwrap().value(), Some(&13));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iterator_backward() {
        let mut root = TrieNode::<usize, 16>::new();
        let mut child3 = TrieNode::<usize, 16>::new();
        child3.value_replace(3);
        let mut child12 = TrieNode::<usize, 16>::new();
        child12.value_replace(12);
        root.child_set(3, child3);
        root.child_set(12, child12);

        let mut i: usize = 0;
        for node in root.into_iter().rev() {
            if i == 0 {
                assert_eq!(node.value(), Some(&12));
            } else if i == 1 {
                assert_eq!(node.value(), Some(&3));
            } else {
                panic!("Only 2 expected");
            }
            i += 1;
        }
        assert_eq!(i, 2);

        let mut iter = &mut root.into_iter();
        assert_eq!(iter.next_back().unwrap().value(), Some(&12));
        assert!(iter.next().is_none());

        let mut iter = &mut root.into_iter();
        assert_eq!(iter.next_back().unwrap().value(), Some(&12));
        assert_eq!(iter.next_back().unwrap().value(), Some(&3));
        assert_eq!(iter.next().unwrap().value(), Some(&12));
        assert!(iter.next().is_none());

        let mut iter = &mut root.into_iter();
        assert_eq!(iter.next_back().unwrap().value(), Some(&12));
        assert_eq!(iter.next_back().unwrap().value(), Some(&3));
        assert_eq!(iter.next().unwrap().value(), Some(&12));
        assert_eq!(iter.next_back().unwrap().value(), Some(&3));
        assert!(iter.next_back().is_none());
    }

    #[test]
    fn test_iterator_from() {
        let mut root = TrieNode::<usize, 16>::new();
        let mut child4 = TrieNode::<usize, 16>::new();
        child4.value_replace(4);
        let mut child11 = TrieNode::<usize, 16>::new();
        child11.value_replace(11);
        root.child_set(4, child4);
        root.child_set(11, child11);

        let mut i: usize = 0;
        for node in root.child_iter_from(6) {
            if i == 0 {
                assert_eq!(node.value(), Some(&11));
            } else {
                panic!("Only 2 expected");
            }
            i += 1;
        }
        assert_eq!(i, 1);

        let mut i: usize = 0;
        for node in root.child_iter_from(6).rev() {
            if i == 0 {
                assert_eq!(node.value(), Some(&4));
            } else {
                panic!("Only 2 expected");
            }
            i += 1;
        }
        assert_eq!(i, 1);

        let mut iter = &mut root.child_iter_from(6);
        assert_eq!(iter.next_back().unwrap().value(), Some(&4));
        assert_eq!(iter.next().unwrap().value(), Some(&11));
        assert!(iter.next().is_none());

        let mut iter = &mut root.child_iter_from(6);
        assert_eq!(iter.next_back().unwrap().value(), Some(&4));
        assert_eq!(iter.next().unwrap().value(), Some(&11));
        assert_eq!(iter.next_back().unwrap().value(), Some(&4));
        assert!(iter.next_back().is_none());

        let mut iter = &mut root.child_iter_from(6);
        assert_eq!(iter.next_back().unwrap().value(), Some(&4));
        assert!(iter.next_back().is_none());
        assert_eq!(iter.next().unwrap().value(), Some(&11));
        assert!(iter.next().is_none());
        assert_eq!(iter.next_back().unwrap().value(), Some(&4));
        assert!(iter.next_back().is_none());
    }
}
