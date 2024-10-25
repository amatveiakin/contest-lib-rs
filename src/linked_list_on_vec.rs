use std::{fmt, ops};


const NODE_MISSING: usize = usize::MAX;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    prev: usize,
    next: usize,
}

// TODO: Consider re-using freed indices. Note that there is a trade-off w.r.t. cache efficiency and
// debuggability (right now accessing a deleted node is a guaranteed panic).
#[derive(Clone)]
pub struct LinkedListOnVec<T> {
    nodes: Vec<Option<Node<T>>>,
    head: usize,
    tail: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedListOnVecIndex(usize);

#[derive(Debug, Clone, Copy)]
pub struct LinkedListOnVecIterator<'a, T> {
    list: &'a LinkedListOnVec<T>,
    cur: LinkedListOnVecIndex,
}

impl<T> LinkedListOnVec<T> {
    pub fn new() -> Self {
        LinkedListOnVec { nodes: Vec::new(), head: NODE_MISSING, tail: NODE_MISSING }
    }

    pub fn is_empty(&self) -> bool {
        self.head == NODE_MISSING
    }
    pub fn len_slow(&self) -> usize {
        self.nodes.iter().filter(|node| node.is_some()).count()
    }

    pub fn head(&self) -> LinkedListOnVecIndex {
        LinkedListOnVecIndex(self.head)
    }
    pub fn tail(&self) -> LinkedListOnVecIndex {
        LinkedListOnVecIndex(self.tail)
    }
    pub fn next(&self, index: LinkedListOnVecIndex) -> LinkedListOnVecIndex {
        LinkedListOnVecIndex(self.nodes[index.0].as_ref().unwrap().next)
    }
    pub fn prev(&self, index: LinkedListOnVecIndex) -> LinkedListOnVecIndex {
        LinkedListOnVecIndex(self.nodes[index.0].as_ref().unwrap().prev)
    }

    pub fn iter(&self) -> LinkedListOnVecIterator<T> {
        LinkedListOnVecIterator { list: self, cur: self.head() }
    }

    pub fn push_back(&mut self, value: T) {
        let new_index = self.nodes.len();
        self.nodes.push(Some(Node { value, prev: self.tail, next: NODE_MISSING }));
        if self.head == NODE_MISSING {
            self.head = new_index;
        } else {
            self.nodes[self.tail].as_mut().unwrap().next = new_index;
        }
        self.tail = new_index;
    }
    pub fn push_front(&mut self, value: T) {
        let new_index = self.nodes.len();
        self.nodes.push(Some(Node { value, prev: NODE_MISSING, next: self.head }));
        if self.tail == NODE_MISSING {
            self.tail = new_index;
        } else {
            self.nodes[self.head].as_mut().unwrap().prev = new_index;
        }
        self.head = new_index;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        (!self.is_empty()).then(|| {
            let node = self.nodes[self.tail].take().unwrap();
            self.tail = node.prev;
            if self.tail == NODE_MISSING {
                self.head = NODE_MISSING;
            } else {
                self.nodes[self.tail].as_mut().unwrap().next = NODE_MISSING;
            }
            node.value
        })
    }
    pub fn pop_front(&mut self) -> Option<T> {
        (!self.is_empty()).then(|| {
            let node = self.nodes[self.head].take().unwrap();
            self.head = node.next;
            if self.head == NODE_MISSING {
                self.tail = NODE_MISSING;
            } else {
                self.nodes[self.head].as_mut().unwrap().prev = NODE_MISSING;
            }
            node.value
        })
    }

    pub fn insert_after(&mut self, index: LinkedListOnVecIndex, value: T) {
        assert!(index.is_valid());
        let new_index = self.nodes.len();
        let prev = index.0;
        let next = self.nodes[prev].as_ref().unwrap().next;
        self.nodes.push(Some(Node { value, prev, next }));
        self.nodes[prev].as_mut().unwrap().next = new_index;
        if next != NODE_MISSING {
            self.nodes[next].as_mut().unwrap().prev = new_index;
        } else {
            assert_eq!(prev, self.tail);
            self.tail = new_index;
        }
    }
    pub fn insert_before(&mut self, index: LinkedListOnVecIndex, value: T) {
        assert!(index.is_valid());
        let new_index = self.nodes.len();
        let prev = self.nodes[index.0].as_ref().unwrap().prev;
        let next = index.0;
        self.nodes.push(Some(Node { value, prev, next }));
        self.nodes[next].as_mut().unwrap().prev = new_index;
        if prev != NODE_MISSING {
            self.nodes[prev].as_mut().unwrap().next = new_index;
        } else {
            assert_eq!(next, self.head);
            self.head = new_index;
        }
    }

    pub fn remove(&mut self, index: LinkedListOnVecIndex) -> T {
        let node = self.nodes[index.0].take().unwrap();
        if node.prev != NODE_MISSING {
            self.nodes[node.prev].as_mut().unwrap().next = node.next;
        } else {
            assert_eq!(index.0, self.head);
            self.head = node.next;
        }
        if node.next != NODE_MISSING {
            self.nodes[node.next].as_mut().unwrap().prev = node.prev;
        } else {
            assert_eq!(index.0, self.tail);
            self.tail = node.prev;
        }
        node.value
    }
}

impl LinkedListOnVecIndex {
    pub fn is_valid(&self) -> bool {
        self.0 != NODE_MISSING
    }
}

impl<T> ops::Index<LinkedListOnVecIndex> for LinkedListOnVec<T> {
    type Output = T;
    fn index(&self, index: LinkedListOnVecIndex) -> &Self::Output {
        &self.nodes[index.0].as_ref().unwrap().value
    }
}
impl<T> ops::IndexMut<LinkedListOnVecIndex> for LinkedListOnVec<T> {
    fn index_mut(&mut self, index: LinkedListOnVecIndex) -> &mut Self::Output {
        &mut self.nodes[index.0].as_mut().unwrap().value
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
        assert_eq!(list.len_slow(), 0);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedListOnVec::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.iter().copied().collect_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedListOnVec::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
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
