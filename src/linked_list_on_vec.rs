// A general-purpose linked list. Comparison to `LinkedListOnRc`:
//   + `LinkedListOnVec` is faster and uses less RAM thanks to vector-based storage.
//   + `LinkedListOnVec` has an O(1) length function.
//   - `LinkedListOnVec` does not support slicing.

use std::{fmt, ops};


const NODE_MISSING: usize = usize::MAX;

// TODO: Consider re-using freed indices. Note that there is a trade-off w.r.t. cache efficiency and
// debuggability (right now accessing a deleted node is a guaranteed panic).
#[derive(Clone)]
pub struct LinkedListOnVec<T> {
    nodes: Vec<Option<T>>,
    prev: Vec<usize>,
    next: Vec<usize>,
    head: usize,
    tail: usize,
    len: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedListIndex(usize);

#[derive(Debug, Clone, Copy)]
pub struct LinkedListOnVecIterator<'a, T> {
    list: &'a LinkedListOnVec<T>,
    cur: LinkedListIndex,
}

impl<T> LinkedListOnVec<T> {
    pub fn new() -> Self {
        LinkedListOnVec {
            nodes: Vec::new(),
            prev: Vec::new(),
            next: Vec::new(),
            head: NODE_MISSING,
            tail: NODE_MISSING,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn head(&self) -> LinkedListIndex {
        LinkedListIndex(self.head)
    }
    pub fn tail(&self) -> LinkedListIndex {
        LinkedListIndex(self.tail)
    }
    pub fn next(&self, index: LinkedListIndex) -> LinkedListIndex {
        LinkedListIndex(self.next[index.0])
    }
    pub fn prev(&self, index: LinkedListIndex) -> LinkedListIndex {
        LinkedListIndex(self.prev[index.0])
    }

    pub fn iter(&self) -> LinkedListOnVecIterator<T> {
        LinkedListOnVecIterator { list: self, cur: self.head() }
    }

    pub fn push_back(&mut self, value: T) {
        if self.is_empty() {
            let index = 0;
            self.nodes.push(Some(value));
            self.prev.push(NODE_MISSING);
            self.next.push(NODE_MISSING);
            self.head = index;
            self.tail = index;
            self.len = 1;
        } else {
            self.insert_after(self.tail(), value);
        }
    }
    pub fn push_front(&mut self, value: T) {
        if self.is_empty() {
            self.push_back(value);
        } else {
            self.insert_before(self.head(), value);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail().is_valid().then(|| self.remove(self.tail()))
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head().is_valid().then(|| self.remove(self.head()))
    }

    pub fn insert_after(&mut self, index: LinkedListIndex, value: T) {
        assert!(index.is_valid());
        let new_index = self.nodes.len();
        let prev = index.0;
        let next = self.next[prev];
        self.nodes.push(Some(value));
        self.next.push(next);
        self.prev.push(prev);
        self.next[prev] = new_index;
        if next != NODE_MISSING {
            self.prev[next] = new_index;
        } else {
            assert_eq!(prev, self.tail);
            self.tail = new_index;
        }
        self.len += 1;
    }
    pub fn insert_before(&mut self, index: LinkedListIndex, value: T) {
        assert!(index.is_valid());
        let new_index = self.nodes.len();
        let prev = self.prev[index.0];
        let next = index.0;
        self.nodes.push(Some(value));
        self.next.push(next);
        self.prev.push(prev);
        self.prev[next] = new_index;
        if prev != NODE_MISSING {
            self.next[prev] = new_index;
        } else {
            assert_eq!(next, self.head);
            self.head = new_index;
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: LinkedListIndex) -> T {
        let prev = self.prev[index.0];
        let next = self.next[index.0];
        if prev != NODE_MISSING {
            self.next[prev] = next;
        } else {
            assert_eq!(index.0, self.head);
            self.head = next;
        }
        if next != NODE_MISSING {
            self.prev[next] = prev;
        } else {
            assert_eq!(index.0, self.tail);
            self.tail = prev;
        }
        self.len -= 1;
        self.nodes[index.0].take().unwrap()
    }
}

impl LinkedListIndex {
    pub fn is_valid(&self) -> bool {
        self.0 != NODE_MISSING
    }
}

impl<T> ops::Index<LinkedListIndex> for LinkedListOnVec<T> {
    type Output = T;
    fn index(&self, index: LinkedListIndex) -> &Self::Output {
        self.nodes[index.0].as_ref().unwrap()
    }
}
impl<T> ops::IndexMut<LinkedListIndex> for LinkedListOnVec<T> {
    fn index_mut(&mut self, index: LinkedListIndex) -> &mut Self::Output {
        self.nodes[index.0].as_mut().unwrap()
    }
}

impl<T> FromIterator<T> for LinkedListOnVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedListOnVec::new();
        for item in iter {
            list.push_back(item);
        }
        list
    }
}

impl<'a, T> Iterator for LinkedListOnVecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.is_valid().then(|| {
            let item = &self.list[self.cur];
            self.cur = self.list.next(self.cur);
            item
        })
    }
}

impl<T> fmt::Debug for LinkedListOnVec<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}


#[cfg(test)]
mod tests {
    use crate::iterutils_basic::IterutilsBasic;

    use super::*;

    #[test]
    fn new() {
        let list: LinkedListOnVec<i32> = LinkedListOnVec::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedListOnVec::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.iter().copied().collect_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedListOnVec::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.iter().copied().collect_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedListOnVec::from_iter([1, 2, 3]);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedListOnVec::from_iter([1, 2, 3]);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn insert_after() {
        let mut list = LinkedListOnVec::from_iter([1, 2]);
        list.insert_after(list.head(), 3);
        list.insert_after(list.tail(), 4);
        assert_eq!(list.iter().copied().collect_vec(), vec![1, 3, 2, 4]);
    }

    #[test]
    fn insert_before() {
        let mut list = LinkedListOnVec::from_iter([1, 2]);
        list.insert_before(list.tail(), 3);
        list.insert_before(list.head(), 4);
        assert_eq!(list.iter().copied().collect_vec(), vec![4, 1, 3, 2]);
    }

    #[test]
    fn remove() {
        let mut list = LinkedListOnVec::from_iter([1, 2, 3]);
        assert_eq!(list.remove(list.next(list.head())), 2);
        assert_eq!(list.iter().copied().collect_vec(), vec![1, 3]);
        assert_eq!(list.remove(list.head()), 1);
        assert_eq!(list.iter().copied().collect_vec(), vec![3]);
        assert_eq!(list.remove(list.head()), 3);
        assert_eq!(list.iter().copied().collect_vec(), vec![]);
        assert!(list.is_empty());
    }

    #[test]
    fn index() {
        let mut list = LinkedListOnVec::from_iter([1, 2]);
        let index = list.tail();
        assert_eq!(list[index], 2);
        list[index] = 3;
        assert_eq!(list[index], 3);
        assert_eq!(list.iter().copied().collect_vec(), vec![1, 3]);
    }

    #[test]
    fn debug_format() {
        let list = LinkedListOnVec::from_iter([1, 2, 3]);
        assert_eq!(format!("{:?}", list), "[1, 2, 3]");
    }
}
