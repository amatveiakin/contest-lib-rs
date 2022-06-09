// Optimization potential: Check index validity in `to_tree_index` and do
// unchecked array access in tree traversal.

use std::ops;

use crate::bits::ceil_to_pow_2;
use crate::u32_index::U32Index;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Node(u32);

impl Node {
    fn is_root(self) -> bool { self.0 == 0 }
    fn left_child(self) -> Node { Node(self.0 * 2 + 1) }
    fn right_child(self) -> Node { Node(self.0 * 2 + 2) }
    fn is_left_child(self) -> bool {
        debug_assert!(!self.is_root());
        self.0 % 2 == 1
    }
    fn parent(self) -> Node {
        debug_assert!(!self.is_root());
        Node((self.0 - 1) / 2)
    }
}


#[derive(Clone, Debug)]
pub struct SegmentTree<T, F> {
    num_leaves: u32,
    num_non_leaves: u32,
    heap: Vec<T>,
    combiner: F,
}

impl<T: Clone + Default, F: Fn(&T, &T) -> T> SegmentTree<T, F> {
    pub fn new(size: u32, combiner: F) -> Self {
        let num_leaves = size;
        let num_non_leaves = ceil_to_pow_2(size) - 1;
        SegmentTree {
            num_leaves,
            num_non_leaves,
            heap: vec![T::default(); (num_non_leaves + num_leaves).try_into().unwrap()],
            combiner,
        }
    }

    pub fn size(&self) -> u32 { self.num_leaves }

    // Updates all values on a segment.
    // Complexity: O(log N) where N is tree size.
    pub fn update(&mut self, idx: impl U32Index, value: &T) {
        let (from, to) = idx.bounds(self.size());
        if from == to {
            return;
        }
        let to_inclusive = to - 1;
        if from == to_inclusive {
            let v = self.to_tree_index(from);
            self.update_vertex(v, value);
            return;
        }
        let mut u = self.to_tree_index(from);
        let mut v = self.to_tree_index(to_inclusive);
        self.update_vertex(u, value);
        self.update_vertex(v, value);
        // We always start at the same depth, so by going one step up simultaneously
        // we'll always arrive to the lowest common parent.
        loop {
            // dbg!(u, v);
            let u_origin = u;
            let v_origin = v;
            u = u.parent();
            v = v.parent();
            if u == v {
                break;
            }
            if u_origin.is_left_child() {
                self.update_vertex(u.right_child(), value);
            }
            if !v_origin.is_left_child() {
                self.update_vertex(v.left_child(), value);
            }
        }
    }

    // Returnes combined value for a segment.
    // Complexity: O(M log N) where M is range length, N is tree size.  // TOOD: Fix
    pub fn get(&self, idx: impl U32Index) -> T {
        let (from, to) = idx.bounds(self.size());
        (from..to).map(|i| self.get_value(i)).fold(Default::default(), |a, b| (self.combiner)(&a, &b))
    }

    // Returns an iterator yielding each value.
    // Complexity of traversing the iterator: O(N log N) where N is tree size.
    // Optimization potential: Make O(N).
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        (0..self.num_leaves).map(|i| self.get_value(i))
    }

    // Equivalent to `self.aggregate(pos, pos + 1)`
    fn get_value(&self, pos: u32) -> T {
        let mut v = self.to_tree_index(pos);
        let mut ret = T::default();
        loop {
            ret = (self.combiner)(&ret, &self.heap[v.0 as usize]);
            if v.0 == 0 {
                return ret;
            }
            v = v.parent();
        }
    }
    fn update_vertex(&mut self, v: Node, value: &T) {
        self.heap[v.0 as usize] = (self.combiner)(&self.heap[v.0 as usize], &value);
    }
    fn to_tree_index(&self, n: u32) -> Node {
        debug_assert!(n < self.num_leaves);
        Node(n + self.num_non_leaves)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn tree_to_vec<T: Clone + Default, F: Fn(&T, &T) -> T>(tree: &SegmentTree<T, F>) -> Vec<T> {
        tree.iter().collect()
    }

    #[test]
    fn update_range() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update(1..6, &42);
        assert_eq!(t.get(3), 42);
        assert_eq!(t.get(7), 0);
        assert_eq!(tree_to_vec(&t), vec![0, 42, 42, 42, 42, 42, 0, 0, 0, 0]);
    }

    #[test]
    fn update_one() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update(3, &17);
        assert_eq!(tree_to_vec(&t), vec![0, 0, 0, 17, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn update_all() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update(.., &33);
        assert_eq!(tree_to_vec(&t), vec![33; 10]);
    }

    #[test]
    fn update_multi() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update(2..7, &1);
        t.update(3..8, &-1);
        t.update(0..6, &10);
        t.update(5..10, &-10);
        assert_eq!(tree_to_vec(&t), vec![10, 10, 11, 10, 10, 0, -10, -11, -10, -10]);
        assert_eq!(t.get(0..2), 20);
        assert_eq!(t.get(1..4), 31);
        assert_eq!(t.get(0..10), 10);
    }

    #[test]
    fn range_syntax() {
        let empty_tree = SegmentTree::<i32, _>::new(5, |x, y| x + y);

        let mut t = empty_tree.clone();
        t.update(1..3, &1);
        assert_eq!(tree_to_vec(&t), vec![0, 1, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(1..=3, &1);
        assert_eq!(tree_to_vec(&t), vec![0, 1, 1, 1, 0]);

        let mut t = empty_tree.clone();
        t.update(..2, &1);
        assert_eq!(tree_to_vec(&t), vec![1, 1, 0, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(..=2, &1);
        assert_eq!(tree_to_vec(&t), vec![1, 1, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(2.., &1);
        assert_eq!(tree_to_vec(&t), vec![0, 0, 1, 1, 1]);

        let mut t = empty_tree.clone();
        t.update(.., &1);
        assert_eq!(tree_to_vec(&t), vec![1, 1, 1, 1, 1]);
    }

}
