pub(crate) struct TrieNode<T> {
    value: Option<T>,
    next: [Option<Box<TrieNode<T>>>; 16],
}

impl<T> TrieNode<T> {
    #[must_use]
    pub(crate) fn new() -> TrieNode<T> {
        TrieNode {
            value: const { None },
            next: [const { None }; 16],
        }
    }

    pub(crate) fn count_children(&self) -> usize {
        let mut count = 0;
        for i in 0..16 {
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

    pub(crate) fn value_mut(&mut self) -> &mut Option<T> {
        &mut self.value
    }

    pub(crate) fn next(&self) -> &[Option<Box<TrieNode<T>>>; 16] {
        &self.next
    }

    pub(crate) fn next_mut(&mut self) -> &mut [Option<Box<TrieNode<T>>>; 16] {
        &mut self.next
    }

    pub(crate) fn child(&self, index: usize) -> Option<&TrieNode<T>> {
        self.next[index].as_deref()
    }

    pub(crate) fn child_mut(&mut self, index: usize) -> Option<&mut TrieNode<T>> {
        self.next[index].as_deref_mut()
    }

    pub(crate) fn child_delete(&mut self, index: usize) -> Option<TrieNode<T>> {
        Some(*self.next[index].take()?)
    }

    pub(crate) fn child_replace(&mut self, index: usize, node: TrieNode<T>) -> Option<TrieNode<T>> {
        Some(*(self.next[index].replace(Box::new(node))?))
    }
}