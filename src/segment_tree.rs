// Improvement potential: Support different combiners for `update` and `get`.
// Improvement potential: Tailored implementations when either queries or updates are
//   always single-value. These could be faster, have smaller memoty footprint and
//   don't required combiner to support iterated application.
// Optimization potential: Rely on index validity check in `U32Index::bound` and do
//   unchecked array access in tree traversal.

use std::ops;

use crate::bits::ceil_to_pow_2;
use crate::u32_index::U32Index;


#[derive(Clone, Debug)]
pub struct SegmentTree<T, F> {
    num_leaves: u32,
    num_non_leaves: u32,
    heap: Vec<Vertex<T>>,
    combiner: F,
    neutral: T,
}

impl<T: Clone, F: Fn(&T, &T, i32) -> T> SegmentTree<T, F> {
    // `combiner` takes (A, B, N) and returns A with B applied N times.
    // Requirement:  combiner(A, B, N) == combiner(...combiner(A, B, 1), ..., B, 1)  N times.
    // Requirement:  combiner(A, neutral, N) == A
    //
    // Example. To compute sum:
    //   * neutral == 0
    //   * combiner: (A, B, N) -> A + B * N
    //
    pub fn new(data: &Vec<T>, neutral: T, combiner: F) -> Self {
        // Shallow leaves are those one level closer to the root than deep leaves.
        // Example. "N" are non-leaves, "S" are shallow leaves, "D" are deep leaves:
        //
        //          ___ N ___
        //         /         \
        //        N           N
        //      /   \       /   \
        //     N     S     S     S
        //   /   \
        //  D     D

        let num_leaves: u32 = data.len().try_into().unwrap();
        let num_shallow_leaves = ceil_to_pow_2(num_leaves) - num_leaves;
        let num_deep_leaves = num_leaves - num_shallow_leaves;
        let num_non_leaves = num_leaves - 1;
        let heap_size = num_non_leaves + num_leaves;
        // Optimization potential: Construct heap in place.
        let mut heap = vec![None; heap_size.try_into().unwrap()];
        Self::fill_heap(
            &data, &neutral, &combiner,
            num_non_leaves, num_shallow_leaves, num_deep_leaves,
            VertexId::root(), &mut heap
        );
        SegmentTree {
            num_leaves,
            num_non_leaves,
            heap: heap.into_iter().map(|option| option.unwrap()).collect(),
            combiner,
            neutral,
        }
    }

    pub fn len(&self) -> u32 { self.num_leaves }

    // Updates each value on a segment.
    // Complexity: O(log N) where N is tree size.
    pub fn update(&mut self, idx: impl U32Index, value: &T) {
        let (from, to) = idx.bounds(self.len());
        self.update_impl(VertexId::root(), from, to, value);
    }

    // Returns combined value for a segment.
    // Complexity: O(log N) where N is tree size.
    pub fn get(&mut self, idx: impl U32Index) -> T {
        let (from, to) = idx.bounds(self.len());
        self.get_impl(VertexId::root(), from, to)
    }

    // Returns an iterator yielding each value separately.
    // Complexity of traversing the iterator: O(N log N) where N is tree size.
    // Optimization potential: Make O(N): push everything down and iterate over leaves.
    pub fn iter(&mut self) -> impl Iterator<Item = T> + '_ {
        (0..self.num_leaves).map(|i| self.get(i))
    }

    fn update_impl(&mut self, v: VertexId, from: u32, to: u32, value: &T) {
        let current = &self.heap[v.0 as usize];
        match vertex_in_segment(&current, from, to) {
            Containment::Full => {
                self.apply_update(v, value);
            },
            Containment::Partial => {
                self.push_down(v);
                self.update_impl(v.left_child(), from, to, value);
                self.update_impl(v.right_child(), from, to, value);
                let answer = (self.combiner)(
                    &self.heap[v.left_child().0 as usize].answer,
                    &self.heap[v.right_child().0 as usize].answer,
                    1
                );
                let current = &mut self.heap[v.0 as usize];
                current.answer = answer;
            },
            Containment::None => {},
        }
    }

    fn get_impl(&mut self, v: VertexId, from: u32, to: u32) -> T {
        let current = &self.heap[v.0 as usize];
        match vertex_in_segment(&current, from, to) {
            Containment::Full => {
                let current = &self.heap[v.0 as usize];
                current.answer.clone()
            },
            Containment::Partial => {
                self.push_down(v);
                let answer_left = self.get_impl(v.left_child(), from, to);
                let answer_right = self.get_impl(v.right_child(), from, to);
                (self.combiner)(&answer_left, &answer_right, 1)
            },
            Containment::None => {
                self.neutral.clone()
            },
        }
    }

    fn is_leaf(&self, v: VertexId) -> bool { v.0 >= self.num_non_leaves }

    fn apply_update(&mut self, v: VertexId, value: &T) {
        let vertex = &mut self.heap[v.0 as usize];
        let subtree_len = (vertex.subtree_end - vertex.subtree_begin).try_into().unwrap();
        vertex.update = (self.combiner)(&vertex.update, &value, 1);
        vertex.answer = (self.combiner)(&vertex.answer, &value, subtree_len);
    }

    fn push_down(&mut self, v: VertexId) {
        let is_leaf = self.is_leaf(v);
        if !is_leaf {
            let current = &self.heap[v.0 as usize];
            let current_update = current.update.clone();
            self.apply_update(v.left_child(), &current_update);
            self.apply_update(v.right_child(), &current_update);
        }
        let current = &mut self.heap[v.0 as usize];
        current.update = self.neutral.clone();
    }

    fn fill_heap(
        data: &Vec<T>, neutral: &T, combiner: &F,
        num_non_leaves: u32, num_shallow_leaves: u32, num_deep_leaves: u32,
        v: VertexId, heap: &mut Vec<Option<Vertex<T>>>)
    {
        if heap[v.0 as usize].is_some() {
            return;
        }
        let is_leaf = v.0 >= num_non_leaves;
        let vertex = if is_leaf {
            let index = v.0 - num_non_leaves;
            let index = if index >= num_shallow_leaves {
                index - num_shallow_leaves
            } else {
                index + num_deep_leaves
            };
            Vertex {
                subtree_begin: index,
                subtree_end: index + 1,
                answer: data[index as usize].clone(),
                update: neutral.clone(),
            }
        } else {
            Self::fill_heap(data, neutral, combiner, num_non_leaves, num_shallow_leaves, num_deep_leaves, v.left_child(), heap);
            Self::fill_heap(data, neutral, combiner, num_non_leaves, num_shallow_leaves, num_deep_leaves, v.right_child(), heap);
            let left_child = &heap[v.left_child().0 as usize].as_ref().unwrap();
            let right_child = &heap[v.right_child().0 as usize].as_ref().unwrap();
            Vertex {
                subtree_begin: left_child.subtree_begin,
                subtree_end: right_child.subtree_end,
                answer: combiner(&left_child.answer, &right_child.answer, 1),
                update: neutral.clone(),
            }
        };
        heap[v.0 as usize] = Some(vertex);
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct VertexId(u32);

impl VertexId {
    fn root() -> Self { VertexId(0) }
    fn left_child(self) -> VertexId { VertexId(self.0 * 2 + 1) }
    fn right_child(self) -> VertexId { VertexId(self.0 * 2 + 2) }
}

#[derive(Clone, Debug)]
struct Vertex<T> {
    subtree_begin: u32,  // minimum index of subtree elements, inclusive
    subtree_end: u32,    // maximum index of subtree elements, exclusive
    answer: T,  // combined value for the entire subtree, after applying `update`s from all parents
                //   (`self.update` is already taken into account!)
    update: T,  // update that should be applied to each subtree element
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Containment {
    Full,
    Partial,
    None,
}

fn vertex_in_segment<T>(v: &Vertex<T>, from: u32, to: u32) -> Containment {
    debug_assert!(from <= to);
    debug_assert!(v.subtree_begin < v.subtree_end);
    if from <= v.subtree_begin && v.subtree_end <= to {
        Containment::Full
    } else if v.subtree_end <= from || to <= v.subtree_begin {
        Containment::None
    } else {
        Containment::Partial
    }
}


#[cfg(test)]
mod tests {
    // TODO: Stress test against naive implementation.
    use std::cmp;
    use super::*;

    fn tree_to_vec<T: Clone, F: Fn(&T, &T, i32) -> T>(tree: &mut SegmentTree<T, F>) -> Vec<T> {
        tree.iter().collect()
    }

    #[test]
    fn one_update() {
        let mut t = SegmentTree::<i32, _>::new(&vec![0; 10], 0, |x, y, n| x + y * n);
        t.update(1..6, &42);
        assert_eq!(t.get(3), 42);
        assert_eq!(t.get(7), 0);
        assert_eq!(t.get(3..7), 126);
        assert_eq!(tree_to_vec(&mut t), vec![0, 42, 42, 42, 42, 42, 0, 0, 0, 0]);
    }

    #[test]
    fn multiple_updates() {
        let mut t = SegmentTree::<i32, _>::new(&vec![0; 10], 0, |x, y, n| x + y * n);
        t.update(2..7, &1);
        t.update(3..8, &-1);
        t.update(0..6, &10);
        t.update(5..10, &-10);
        assert_eq!(tree_to_vec(&mut t), vec![10, 10, 11, 10, 10, 0, -10, -11, -10, -10]);
        assert_eq!(t.get(0..2), 20);
        assert_eq!(t.get(1..4), 31);
        assert_eq!(t.get(0..10), 10);
    }

    #[test]
    fn custom_init() {
        let v = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let mut t = SegmentTree::<i32, _>::new(&v, 0, |x, y, n| x + y * n);
        assert_eq!(tree_to_vec(&mut t), v);
        assert_eq!(t.get(..3), 4);
        assert_eq!(t.get(2..5), 10);
        assert_eq!(t.get(..), 143);
    }

    // A tree of 2^N elements is a special case for shallow leaves / deep leaves calculations.
    #[test]
    fn pow_of_two_size() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut t = SegmentTree::<i32, _>::new(&v, 0, |x, y, n| x + y * n);
        assert_eq!(tree_to_vec(&mut t), v);
        t.update(3..=4, &-10);
        t.update(2..=5, &5);
        assert_eq!(t.get(1..=6), 27);
        assert_eq!(t.get(4), 0);
    }

    #[test]
    fn min_tree() {
        let mut t = SegmentTree::<i32, _>::new(&vec![i32::MAX; 7], i32::MAX, |x, y, _n| cmp::min(*x, *y));
        t.update(1..=3, &55);
        t.update(3..=4, &77);
        t.update(2, &99);
        assert_eq!(tree_to_vec(&mut t), vec![i32::MAX, 55, 55, 55, 77, i32::MAX, i32::MAX]);
    }

    #[test]
    fn range_syntax() {
        let empty_tree = SegmentTree::<i32, _>::new(&vec![0; 5], 0, |x, y, n| x + y * n);

        let mut t = empty_tree.clone();
        t.update(2, &1);
        assert_eq!(tree_to_vec(&mut t), vec![0, 0, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(1..3, &1);
        assert_eq!(tree_to_vec(&mut t), vec![0, 1, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(1..=3, &1);
        assert_eq!(tree_to_vec(&mut t), vec![0, 1, 1, 1, 0]);

        let mut t = empty_tree.clone();
        t.update(..2, &1);
        assert_eq!(tree_to_vec(&mut t), vec![1, 1, 0, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(..=2, &1);
        assert_eq!(tree_to_vec(&mut t), vec![1, 1, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(2.., &1);
        assert_eq!(tree_to_vec(&mut t), vec![0, 0, 1, 1, 1]);

        let mut t = empty_tree.clone();
        t.update(.., &1);
        assert_eq!(tree_to_vec(&mut t), vec![1, 1, 1, 1, 1]);
    }
}
