// A general-purpose linked list. Comparison to `LinkedListOnVec`:
//   - `LinkedListOnRc` is slower and uses more RAM due to heap-based storage.
//   - `LinkedListOnRc` does not know its length.
//   + `LinkedListOnRc` supports O(1) slicing.

use std::cell::{Ref, RefCell, RefMut};
use std::{fmt, mem};
use std::rc::Rc;


struct LinkedListNodeImpl<T> {
    value: Option<T>,  // always non-null unless the node is removed
    prev: Option<LinkedListNode<T>>,
    next: Option<LinkedListNode<T>>,
}

pub struct LinkedListNode<T>(Rc<RefCell<LinkedListNodeImpl<T>>>);

pub struct LinkedListOnRc<T> {
    head: Option<LinkedListNode<T>>,
    tail: Option<LinkedListNode<T>>,
}

pub struct LinkedListOnRcClonedIterator<T> {
    cur: Option<LinkedListNode<T>>,
}

pub struct LinkedListOnRcNodeIterator<T> {
    cur: Option<LinkedListNode<T>>,
}

impl<T> LinkedListOnRc<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn head(&self) -> Option<LinkedListNode<T>> {
        self.head.clone()
    }
    pub fn tail(&self) -> Option<LinkedListNode<T>> {
        self.tail.clone()
    }

    // IIUC there is not way to return a reference from the iterator because we cannot be sure that
    // the value will outlive the container. In order to iterate over references, use `iter_nodes`
    // and call `get` manually.
    pub fn iter_cloned(&self) -> LinkedListOnRcClonedIterator<T> {
        LinkedListOnRcClonedIterator { cur: self.head() }
    }
    pub fn iter_nodes(&self) -> LinkedListOnRcNodeIterator<T> {
        LinkedListOnRcNodeIterator { cur: self.head() }
    }

    pub fn push_back(&mut self, value: T) {
        if let Some(tail) = self.tail.clone() {
            self.insert_after(&tail, value);
        } else {
            let node = LinkedListNode::new(value, None, None);
            self.head = Some(node.clone());
            self.tail = Some(node);
        }
    }
    pub fn push_front(&mut self, value: T) {
        if let Some(head) = self.head.clone() {
            self.insert_before(&head, value);
        } else {
            let node = LinkedListNode::new(value, None, None);
            self.head = Some(node.clone());
            self.tail = Some(node);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.clone().map(|node| self.remove(node))
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.clone().map(|node| self.remove(node))
    }

    pub fn insert_after(&mut self, pivot: &LinkedListNode<T>, value: T) {
        let new_node = LinkedListNode::new(value, Some(pivot.clone()), pivot.next());
        if let Some(next) = pivot.next() {
            next.set_prev(Some(new_node.clone()));
        }
        if Rc::ptr_eq(&self.tail.as_ref().unwrap().0, &pivot.0) {
            self.tail = Some(new_node.clone());
        }
        pivot.set_next(Some(new_node));
    }
    pub fn insert_before(&mut self, pivot: &LinkedListNode<T>, value: T) {
        let new_node = LinkedListNode::new(value, pivot.prev(), Some(pivot.clone()));
        if let Some(prev) = pivot.prev() {
            prev.set_next(Some(new_node.clone()));
        }
        if Rc::ptr_eq(&self.head.as_ref().unwrap().0, &pivot.0) {
            self.head = Some(new_node.clone());
        }
        pivot.set_prev(Some(new_node));
    }

    // Note. The node is destroyed after the removal. Even if you have another copy of the node,
    // you cannot use it to access the value after removal.
    pub fn remove(&mut self, node: LinkedListNode<T>) -> T {
        let mut borrow = node.0.borrow_mut();
        let next = borrow.next.take();
        let prev = borrow.prev.take();
        if Rc::ptr_eq(&self.head.as_ref().unwrap().0, &node.0) {
            self.head = next.clone();
        }
        if Rc::ptr_eq(&self.tail.as_ref().unwrap().0, &node.0) {
            self.tail = prev.clone();
        }
        if let Some(prev) = prev.clone() {
            prev.set_next(next.clone());
        }
        if let Some(next) = next {
            next.set_prev(prev);
        }
        borrow.value.take().unwrap()
    }

    // Both ends are inclusive.
    pub fn extract_range(
        &mut self, from: LinkedListNode<T>, to: LinkedListNode<T>
    ) -> LinkedListOnRc<T> {
        if Rc::ptr_eq(&self.head.as_ref().unwrap().0, &from.0) {
            self.head = to.next();
        }
        if Rc::ptr_eq(&self.tail.as_ref().unwrap().0, &to.0) {
            self.tail = from.prev();
        }
        if let Some(prev) = from.prev() {
            prev.set_next(to.next());
        }
        if let Some(next) = to.next() {
            next.set_prev(from.prev());
        }
        from.set_prev(None);
        to.set_next(None);
        LinkedListOnRc {
            head: Some(from),
            tail: Some(to),
        }
    }

    // To splice a range of nodes, use `extract_range` first.
    pub fn splice_after(&mut self, pivot: &LinkedListNode<T>, list: LinkedListOnRc<T>) {
        if list.is_empty() {
            return;
        }
        let from = list.head().unwrap();
        let to = list.tail().unwrap();
        mem::forget(list);
        from.set_prev(Some(pivot.clone()));
        to.set_next(pivot.next());
        if let Some(next) = pivot.next() {
            next.set_prev(Some(to.clone()));
        }
        if Rc::ptr_eq(&self.tail.as_ref().unwrap().0, &pivot.0) {
            self.tail = Some(to);
        }
        pivot.set_next(Some(from));
    }
    pub fn splice_before(&mut self, pivot: &LinkedListNode<T>, list: LinkedListOnRc<T>) {
        if list.is_empty() {
            return;
        }
        let from = list.head().unwrap();
        let to = list.tail().unwrap();
        mem::forget(list);
        from.set_prev(pivot.prev());
        to.set_next(Some(pivot.clone()));
        if let Some(prev) = pivot.prev() {
            prev.set_next(Some(from.clone()));
        }
        if Rc::ptr_eq(&self.head.as_ref().unwrap().0, &pivot.0) {
            self.head = Some(from);
        }
        pivot.set_prev(Some(to));
    }
}

impl<T> LinkedListNode<T> {
    // Returns true if the node is alive, false if the node was removed.
    // Other operations on a removed node could panic.
    pub fn is_alive(&self) -> bool {
        self.0.borrow().value.is_some()
    }

    pub fn get(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |node| node.value.as_ref().unwrap())
    }
    pub fn get_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |node| node.value.as_mut().unwrap())
    }

    pub fn next(&self) -> Option<LinkedListNode<T>> {
        self.0.borrow().next.clone()
    }
    pub fn prev(&self) -> Option<LinkedListNode<T>> {
        self.0.borrow().prev.clone()
    }

    fn new(value: T, prev: Option<LinkedListNode<T>>, next: Option<LinkedListNode<T>>) -> Self {
        LinkedListNode(Rc::new(RefCell::new(LinkedListNodeImpl { value: Some(value), prev, next })))
    }
    fn set_next(&self, next: Option<LinkedListNode<T>>) {
        self.0.borrow_mut().next = next;
    }
    fn set_prev(&self, prev: Option<LinkedListNode<T>>) {
        self.0.borrow_mut().prev = prev;
    }
}

impl<T> Clone for LinkedListNode<T> {
    fn clone(&self) -> Self {
        LinkedListNode(Rc::clone(&self.0))
    }
}

impl<T> Drop for LinkedListOnRc<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

impl<T> FromIterator<T> for LinkedListOnRc<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push_back(item);
        }
        list
    }
}

impl<T: Clone> Iterator for LinkedListOnRcClonedIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            self.cur = node.next();
            node.get().clone()
        })
    }
}

impl<T> Iterator for LinkedListOnRcNodeIterator<T> {
    type Item = LinkedListNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            self.cur = node.next();
            node
        })
    }
}

impl<T> fmt::Debug for LinkedListOnRc<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        let mut next = self.head();
        while let Some(cur) = next {
            list.entry(&*cur.get());
            next = cur.next();
        }
        list.finish()
    }
}


#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::iterutils_basic::IterutilsBasic;

    use super::*;

    #[test]
    fn new() {
        let list: LinkedListOnRc<i32> = LinkedListOnRc::new();
        assert!(list.is_empty());
        assert_eq!(list.iter_cloned().count(), 0);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedListOnRc::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.iter_cloned().collect_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedListOnRc::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.iter_cloned().collect_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedListOnRc::from_iter([1, 2, 3]);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedListOnRc::from_iter([1, 2, 3]);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn insert_after() {
        let mut list = LinkedListOnRc::from_iter([1, 2]);
        list.insert_after(&list.head().unwrap(), 3);
        list.insert_after(&list.tail().unwrap(), 4);
        assert_eq!(list.iter_cloned().collect_vec(), vec![1, 3, 2, 4]);
    }

    #[test]
    fn insert_before() {
        let mut list = LinkedListOnRc::from_iter([1, 2]);
        list.insert_before(&list.tail().unwrap(), 3);
        list.insert_before(&list.head().unwrap(), 4);
        assert_eq!(list.iter_cloned().collect_vec(), vec![4, 1, 3, 2]);
    }

    #[test]
    fn remove() {
        let mut list = LinkedListOnRc::from_iter([1, 2, 3]);
        assert_eq!(list.remove(list.head().unwrap().next().unwrap()), 2);
        assert_eq!(list.iter_cloned().collect_vec(), vec![1, 3]);
        assert_eq!(list.remove(list.head().unwrap()), 1);
        assert_eq!(list.iter_cloned().collect_vec(), vec![3]);
        assert_eq!(list.remove(list.head().unwrap()), 3);
        assert_eq!(list.iter_cloned().collect_vec(), vec![]);
        assert!(list.is_empty());
    }

    #[test]
    fn node() {
        let list = LinkedListOnRc::from_iter([1, 2]);
        let node = list.tail().unwrap();
        assert_eq!(*node.get(), 2);
        *node.get_mut() = 3;
        assert_eq!(*node.get(), 3);
        assert_eq!(list.iter_cloned().collect_vec(), vec![1, 3]);
    }

    #[test]
    fn extract_front() {
        let mut a = LinkedListOnRc::from_iter([1, 2, 3, 4]);
        let b = a.extract_range(
            a.head().unwrap(),
            a.head().unwrap().next().unwrap(),
        );
        assert_eq!(b.iter_cloned().collect_vec(), vec![1, 2]);
        assert_eq!(a.iter_cloned().collect_vec(), vec![3, 4]);
    }

    #[test]
    fn extract_back() {
        let mut a = LinkedListOnRc::from_iter([1, 2, 3, 4]);
        let b = a.extract_range(
            a.tail().unwrap().prev().unwrap(),
            a.tail().unwrap(),
        );
        assert_eq!(b.iter_cloned().collect_vec(), vec![3, 4]);
        assert_eq!(a.iter_cloned().collect_vec(), vec![1, 2]);
    }

    #[test]
    fn extract_middle() {
        let mut a = LinkedListOnRc::from_iter([1, 2, 3, 4]);
        let b = a.extract_range(
            a.head().unwrap().next().unwrap(),
            a.tail().unwrap().prev().unwrap(),
        );
        assert_eq!(b.iter_cloned().collect_vec(), vec![2, 3]);
        assert_eq!(a.iter_cloned().collect_vec(), vec![1, 4]);
    }

    #[test]
    fn extract_one() {
        let mut a = LinkedListOnRc::from_iter([1, 2, 3, 4]);
        let b = a.extract_range(
            a.head().unwrap().next().unwrap(),
            a.head().unwrap().next().unwrap(),
        );
        assert_eq!(b.iter_cloned().collect_vec(), vec![2]);
        assert_eq!(a.iter_cloned().collect_vec(), vec![1, 3, 4]);
    }

    #[test]
    fn extract_all() {
        let mut a = LinkedListOnRc::from_iter([1, 2, 3, 4]);
        let b = a.extract_range(
            a.head().unwrap(),
            a.tail().unwrap(),
        );
        assert_eq!(b.iter_cloned().collect_vec(), vec![1, 2, 3, 4]);
        assert!(a.is_empty());
    }

    #[test]
    fn splice_after_middle() {
        let mut a = LinkedListOnRc::from_iter([1, 2]);
        let b = LinkedListOnRc::from_iter([3, 4]);
        a.splice_after(&a.head().unwrap(), b);
        assert_eq!(a.iter_cloned().collect_vec(), vec![1, 3, 4, 2]);
    }

    #[test]
    fn splice_after_tail() {
        let mut a = LinkedListOnRc::from_iter([1, 2]);
        let b = LinkedListOnRc::from_iter([3, 4]);
        a.splice_after(&a.tail().unwrap(), b);
        assert_eq!(a.iter_cloned().collect_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn splice_before_middle() {
        let mut a = LinkedListOnRc::from_iter([1, 2]);
        let b = LinkedListOnRc::from_iter([3, 4]);
        a.splice_before(&a.tail().unwrap(), b);
        assert_eq!(a.iter_cloned().collect_vec(), vec![1, 3, 4, 2]);
    }

    #[test]
    fn splice_before_head() {
        let mut a = LinkedListOnRc::from_iter([1, 2]);
        let b = LinkedListOnRc::from_iter([3, 4]);
        a.splice_before(&a.head().unwrap(), b);
        assert_eq!(a.iter_cloned().collect_vec(), vec![3, 4, 1, 2]);
    }

    #[test]
    fn no_leaks() {
        let num_drops = Rc::new(Cell::new(0));
        struct Node(Rc<Cell<usize>>);
        impl Drop for Node {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let list = LinkedListOnRc::from_iter(vec![
            Node(num_drops.clone()),
            Node(num_drops.clone()),
        ]);
        drop(list);
        assert_eq!(num_drops.get(), 2);
    }

    #[test]
    fn debug_format() {
        let list = LinkedListOnRc::from_iter([1, 2, 3]);
        assert_eq!(format!("{:?}", list), "[1, 2, 3]");
    }
}
