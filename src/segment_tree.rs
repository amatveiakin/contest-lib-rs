// Optimization potential: Check index validity in `to_tree_index` and do
// unchecked array access in tree traversal.

use crate::bits::ceil_to_pow_2;


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


pub struct SegmentTree<T: Clone + Default, F: Fn(&T, &T) -> T> {
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

    pub fn update(&mut self, pos: u32, value: &T) {
        let v = self.to_tree_index(pos);
        self.update_vertex(v, value);
    }
    // Improvement potential: Accept ranges instead.
    // Updates segment [`from`, `to`).
    pub fn update_range(&mut self, from: u32, to: u32, value: &T) {
        if from == to {
            return;
        }
        let to_inclusive = to - 1;
        if from == to_inclusive {
            self.update(from, value);
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

    // Improvement potential: Implement via index operator; accept ranges.
    // Equivalent to `self.aggregate(pos, pos + 1)`
    pub fn value(&self, pos: u32) -> T {
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
    // Returnes combined value for segment [`from`, `to`).
    // Optimization potential: Make O(N) instead of O(N log N).
    pub fn aggregate(&self, from: u32, to: u32) -> T {
        (from..to).map(|i| self.value(i)).fold(Default::default(), |a, b| (self.combiner)(&a, &b))
    }

    // Optimization potential: Make O(N) instead of O(N log N).
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        (0..self.num_leaves).map(|i| self.value(i))
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
        t.update_range(1, 6, &42);
        assert_eq!(t.value(3), 42);
        assert_eq!(t.value(7), 0);
        assert_eq!(tree_to_vec(&t), vec![0, 42, 42, 42, 42, 42, 0, 0, 0, 0]);
    }

    #[test]
    fn update_one() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update(3, &17);
        t.update_range(5, 6, &18);
        assert_eq!(tree_to_vec(&t), vec![0, 0, 0, 17, 0, 18, 0, 0, 0, 0]);
    }

    #[test]
    fn update_all() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update_range(0, 10, &33);
        assert_eq!(tree_to_vec(&t), vec![33; 10]);
    }

    #[test]
    fn update_multi() {
        let mut t = SegmentTree::<i32, _>::new(10, |x, y| x + y);
        t.update_range(2, 7, &1);
        t.update_range(3, 8, &-1);
        t.update_range(0, 6, &10);
        t.update_range(5, 10, &-10);
        assert_eq!(tree_to_vec(&t), vec![10, 10, 11, 10, 10, 0, -10, -11, -10, -10]);
        assert_eq!(t.aggregate(0, 2), 20);
        assert_eq!(t.aggregate(1, 4), 31);
        assert_eq!(t.aggregate(0, 10), 10);
    }
}
