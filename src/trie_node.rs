pub(crate) struct TrieNode<T, const N: usize> {
    value: Option<T>,
    next: [Option<Box<TrieNode<T, N>>>; N],
}

impl<T, const N: usize> TrieNode<T, N> {
    #[must_use]
    pub(crate) fn new() -> TrieNode<T, N> {
        TrieNode {
            value: const { None },
            next: [const { None }; N],
        }
    }

    pub(crate) fn has_child(&self) -> bool {
        for i in 0..N {
            if self.next[i].is_some() {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_multiple_children(&self) -> bool {
        let mut count = 0;
        for i in 0..N {
            if self.next[i].is_some() {
                if count > 0 {
                    return true;
                }
                count += 1;
            }
        }
        false
    }

    pub(crate) fn count_children(&self) -> usize {
        let mut count = 0;
        for i in 0..N {
            if self.next[i].is_some() {
                count += 1;
            }
        }
        count
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

    pub(crate) fn child_take(&mut self, index: usize) -> Option<TrieNode<T, N>> {
        Some(*self.next[index].take()?)
    }

    pub(crate) fn child_replace(&mut self, index: usize, node: TrieNode<T, N>) -> Option<TrieNode<T, N>> {
        Some(*(self.next[index].replace(Box::new(node))?))
    }

    pub(crate) fn child_set(&mut self, index: usize, node: TrieNode<T, N>) -> &mut TrieNode<T, N> {
        self.next[index].insert(Box::new(node))
    }
}