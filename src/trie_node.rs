use slotmap::{DefaultKey, SlotMap};

pub(crate) struct TrieNode<T, const N: usize> {
    value: Option<T>,
    next: [Option<DefaultKey>; N],
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

    pub(crate) fn child_key(&self, index: usize) -> Option<DefaultKey> {
        self.next[index]
    }

    pub(crate) fn child_remove(&mut self, index: usize) {
        self.next[index] = None;
    }

    pub(crate) fn child_set(&mut self, index: usize, key: DefaultKey) {
        self.next[index] = Some(key);
    }
}
