// Lazy heterogenous segment tree - basically, the most generic segment tree I could think of.
//
// The tree stores values of type `T` and applies updates of type `U`. In order to construct the
// tree you need:
//   - Neutral value (returned when querying an empty range).
//   - Neutral update (one that doesn't change the value).
//   - How to combine two values into a new value.
//   - How to combine two updates into a new update.
//   - How to apply an update to a range of values. Note the "range" part: the `applier` function
//     tells how the result of `get` changes for any possible range after an update is applied. E.g.
//     if the tree computes sums on a segment and the update adds `x` to each value on a segment,
//     then `applier(old_value, x, l, r)` is not `old_value + x`, but `old_value + x * (r - l)`.
//
// In the special case where value type conincides with update type and `applier` does not take
// element position into account, the tree could be constructed via `new_homogenous_tree`. This
// constructor takes a single callback `(a, b, n) -> ret` which answers the question "How would
// value `a` changes after applying update `b` to it `n` times?"
//
// There are also several predefined trees:
//   - `new_sum_tree`: Query computes sum of a segment. Update(x) adds x to each value.
//   - `new_min_tree`: Query computes minimum on a segment. Update(x) replaces y with min(x, y).
//   - `new_max_tree`: Query computes maximum on a segment. Update(x) replaces y with max(x, y).
//
// On neutral values and updates. Both are, strictly speaking, unnecessary, however:
//   - Neutral value is returned when querying and empty range.
//   - Neutral update could be removed from the API altogether by storing `Option<Update>`
//     internally. But this would make the common case less optimal, and in practice it's usually
//     easy to specify a neutral update. If there is no neutral update value, it could be emulated
//     by using `Option<Update>`; see `optional_update` test for an example.
//
// Improvement potential: Replace slices with `IntoIterator`s in tree constructors.
// Improvement potential: See if the implementation can be simplified. First, the logic of dealing
//   with non-power-of-two sizes is probably overcomplicated. Second, I'm not sure vertices need to
//   store `subtree_begin`/`subtree_end` indices.
// Improvement potential: Tailored implementations when either queries or updates are always
//   single-value (i.e. non-lazy segment trees). These could be faster, have smaller memory
//   footprint and simpler APIs.
// Optimization potential: Rely on index validity check in `U32Index::bound` and do unchecked array
//   access in tree traversal.

use std::fmt;

use crate::num::{RingInteger, Integer};
use crate::u32_index::U32Index;


// Use this trait instead of the actual tree type to reduce the number of generic parameters in
// functions accepting a segment tree as an argument. Note that all these methods are also present
// directly in the `SegmentTree` class, so that most users don't need to import the trait.
pub trait ISegmentTree<T, U> {
    fn len(&self) -> u32;

    // Updates each value on a segment.
    // Complexity: O(log N) where N is tree size.
    fn update(&mut self, idx: impl U32Index, upd: &U);

    // Returns combined value for a segment.
    // Complexity: O(log N) where N is tree size.
    fn get(&mut self, idx: impl U32Index) -> T;

    // Returns an iterator yielding each value separately.
    // Complexity of traversing the iterator: O(N log N) where N is tree size.
    // Optimization potential: Make O(N): push everything down and iterate over leaves.
    fn iter(&mut self) -> impl DoubleEndedIterator<Item = T> + ExactSizeIterator + '_;
}

pub fn new_homogenous_tree<T: Clone>(data: &[T], neutral: T, combiner: impl Clone + Fn(&T, &T, u32) -> T)
    -> SegmentTree<T, T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T, u32, u32) -> T>
{
    let combiner1 = combiner.clone();
    let combiner2 = combiner.clone();
    let combiner3 = combiner;
    SegmentTree::new(
        data,
        neutral.clone(),
        neutral,
        move |v1, v2| combiner1(v1, v2, 1),
        move |u1, u2| combiner2(u1, u2, 1),
        move |v, u, l, r| combiner3(v, u, (r - l) as u32),
    )
}

pub fn new_sum_tree<T: RingInteger>(data: &[T])
    -> SegmentTree<T, T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T, u32, u32) -> T>
where
    u32: TryInto<T>,
    <u32 as TryInto<T>>::Error: fmt::Debug,
{
    new_homogenous_tree(data, T::zero(), |&a, &b, n| a + b * n.try_into().unwrap())
}

pub fn new_min_tree<T: Integer>(data: &[T])
    -> SegmentTree<T, T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T, u32, u32) -> T>
{
    new_homogenous_tree(data, T::MAX, |&a, &b, _| a.minv(b))
}

pub fn new_max_tree<T: Integer>(data: &[T])
    -> SegmentTree<T, T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T) -> T, impl Clone + Fn(&T, &T, u32, u32) -> T>
{
    new_homogenous_tree(data, T::MIN, |&a, &b, _| a.maxv(b))
}

#[derive(Clone, Debug)]
pub struct SegmentTree<T, U, TC, UC, A> {
    num_leaves: u32,
    num_non_leaves: u32,
    heap: Vec<Vertex<T, U>>,
    neutral_value: T,
    neutral_update: U,
    value_combiner: TC,
    update_combiner: UC,
    applier: A,
}

impl<
    T: Clone,
    U: Clone,
    TC: Fn(&T, &T) -> T,
    UC: Fn(&U, &U) -> U,
    A: Fn(&T, &U, u32, u32) -> T,
> SegmentTree<T, U, TC, UC, A> {
    pub fn new(
        data: &[T], neutral_value: T, neutral_update: U,
        value_combiner: TC, update_combiner: UC, applier: A) -> Self
    {
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
        let num_shallow_leaves = num_leaves.next_power_of_two() - num_leaves;
        let num_deep_leaves = num_leaves - num_shallow_leaves;
        let num_non_leaves = num_leaves - 1;
        let heap_size = num_non_leaves + num_leaves;
        // Optimization potential: Construct heap in place.
        let mut heap = vec![None; heap_size.try_into().unwrap()];
        Self::fill_heap(
            &data, &neutral_update, &value_combiner,
            num_non_leaves, num_shallow_leaves, num_deep_leaves,
            VertexId::root(), &mut heap
        );
        SegmentTree {
            num_leaves,
            num_non_leaves,
            heap: heap.into_iter().map(|option| option.unwrap()).collect(),
            neutral_value,
            neutral_update,
            value_combiner,
            update_combiner,
            applier,
        }
    }

    pub fn len(&self) -> u32 { self.num_leaves }

    pub fn update(&mut self, idx: impl U32Index, upd: &U) {
        let (from, to) = idx.bounds(self.len());
        self.update_impl(VertexId::root(), from, to, upd);
    }

    pub fn get(&mut self, idx: impl U32Index) -> T {
        let (from, to) = idx.bounds(self.len());
        self.get_impl(VertexId::root(), from, to)
    }

    pub fn iter(&mut self) -> impl DoubleEndedIterator<Item = T> + ExactSizeIterator + '_ {
        (0..self.num_leaves).map(|i| self.get(i))
    }

    fn update_impl(&mut self, v: VertexId, from: u32, to: u32, upd: &U) {
        let current = &self.heap[v.0 as usize];
        match vertex_in_segment(&current, from, to) {
            Containment::Full => {
                self.apply_update(v, upd);
            },
            Containment::Partial => {
                self.push_down(v);
                self.update_impl(v.left_child(), from, to, upd);
                self.update_impl(v.right_child(), from, to, upd);
                let answer = (self.value_combiner)(
                    &self.heap[v.left_child().0 as usize].answer,
                    &self.heap[v.right_child().0 as usize].answer,
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
                (self.value_combiner)(&answer_left, &answer_right)
            },
            Containment::None => {
                self.neutral_value.clone()
            },
        }
    }

    fn is_leaf(&self, v: VertexId) -> bool { v.0 >= self.num_non_leaves }

    fn apply_update(&mut self, v: VertexId, upd: &U) {
        let vertex = &mut self.heap[v.0 as usize];
        vertex.update = (self.update_combiner)(&vertex.update, &upd);
        vertex.answer = (self.applier)(&vertex.answer, &upd, vertex.subtree_begin, vertex.subtree_end);
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
        current.update = self.neutral_update.clone();
    }

    fn fill_heap(
        data: &[T], neutral: &U, value_combiner: &TC,
        num_non_leaves: u32, num_shallow_leaves: u32, num_deep_leaves: u32,
        v: VertexId, heap: &mut Vec<Option<Vertex<T, U>>>)
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
            Self::fill_heap(data, neutral, value_combiner, num_non_leaves, num_shallow_leaves, num_deep_leaves, v.left_child(), heap);
            Self::fill_heap(data, neutral, value_combiner, num_non_leaves, num_shallow_leaves, num_deep_leaves, v.right_child(), heap);
            let left_child = &heap[v.left_child().0 as usize].as_ref().unwrap();
            let right_child = &heap[v.right_child().0 as usize].as_ref().unwrap();
            Vertex {
                subtree_begin: left_child.subtree_begin,
                subtree_end: right_child.subtree_end,
                answer: value_combiner(&left_child.answer, &right_child.answer),
                update: neutral.clone(),
            }
        };
        heap[v.0 as usize] = Some(vertex);
    }
}

impl<
    T: Clone,
    U: Clone,
    TC: Fn(&T, &T) -> T,
    UC: Fn(&U, &U) -> U,
    A: Fn(&T, &U, u32, u32) -> T,
> ISegmentTree<T, U> for SegmentTree<T, U, TC, UC, A> {
    fn len(&self) -> u32 { self.len() }
    fn update(&mut self, idx: impl U32Index, upd: &U) { self.update(idx, upd) }
    fn get(&mut self, idx: impl U32Index) -> T { self.get(idx) }
    fn iter(&mut self) -> impl DoubleEndedIterator<Item = T> + ExactSizeIterator + '_ { self.iter() }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct VertexId(u32);

impl VertexId {
    fn root() -> Self { VertexId(0) }
    fn left_child(self) -> VertexId { VertexId(self.0 * 2 + 1) }
    fn right_child(self) -> VertexId { VertexId(self.0 * 2 + 2) }
}

#[derive(Clone, Debug)]
struct Vertex<T, U> {
    subtree_begin: u32,  // minimum index of subtree elements, inclusive
    subtree_end: u32,    // maximum index of subtree elements, exclusive
    answer: T,  // combined value for the entire subtree, after applying `update`s from all parents
                //   (`self.update` is already taken into account!)
    update: U,  // update that should be applied to each subtree element
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Containment {
    Full,
    Partial,
    None,
}

fn vertex_in_segment<T, U>(v: &Vertex<T, U>, from: u32, to: u32) -> Containment {
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
    use crate::iterutils_basic::IterutilsBasic;

    // TODO: Stress test against naive implementation.
    use super::*;

    #[test]
    fn one_update() {
        let mut t = new_sum_tree(&vec![0; 10]);
        t.update(1..6, &42);
        assert_eq!(t.get(3), 42);
        assert_eq!(t.get(7), 0);
        assert_eq!(t.get(3..7), 126);
        assert_eq!(t.iter().collect_vec(), vec![0, 42, 42, 42, 42, 42, 0, 0, 0, 0]);
    }

    #[test]
    fn multiple_updates() {
        let mut t = new_sum_tree(&vec![0; 10]);
        t.update(2..7, &1);
        t.update(3..8, &-1);
        t.update(0..6, &10);
        t.update(5..10, &-10);
        assert_eq!(t.iter().collect_vec(), vec![10, 10, 11, 10, 10, 0, -10, -11, -10, -10]);
        assert_eq!(t.get(0..2), 20);
        assert_eq!(t.get(1..4), 31);
        assert_eq!(t.get(0..10), 10);
    }

    #[test]
    fn custom_init() {
        let v = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let mut t = new_sum_tree(&v);
        assert_eq!(t.iter().collect_vec(), v);
        assert_eq!(t.get(..3), 4);
        assert_eq!(t.get(2..5), 10);
        assert_eq!(t.get(..), 143);
    }

    // A tree of 2^N elements is a special case for shallow leaves / deep leaves calculations.
    #[test]
    fn pow_of_two_size() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut t = new_sum_tree(&v);
        assert_eq!(t.iter().collect_vec(), v);
        t.update(3..=4, &-10);
        t.update(2..=5, &5);
        assert_eq!(t.get(1..=6), 27);
        assert_eq!(t.get(4), 0);
    }

    #[test]
    fn min_tree() {
        let mut t = new_min_tree(&vec![i32::MAX; 7]);
        t.update(1..=3, &55);
        t.update(3..=4, &77);
        t.update(2, &99);
        assert_eq!(t.iter().collect_vec(), vec![i32::MAX, 55, 55, 55, 77, i32::MAX, i32::MAX]);
    }

    #[test]
    fn range_syntax() {
        let empty_tree = new_sum_tree(&vec![0; 5]);

        let mut t = empty_tree.clone();
        t.update(2, &1);
        assert_eq!(t.iter().collect_vec(), vec![0, 0, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(1..3, &1);
        assert_eq!(t.iter().collect_vec(), vec![0, 1, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(1..=3, &1);
        assert_eq!(t.iter().collect_vec(), vec![0, 1, 1, 1, 0]);

        let mut t = empty_tree.clone();
        t.update(..2, &1);
        assert_eq!(t.iter().collect_vec(), vec![1, 1, 0, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(..=2, &1);
        assert_eq!(t.iter().collect_vec(), vec![1, 1, 1, 0, 0]);

        let mut t = empty_tree.clone();
        t.update(2.., &1);
        assert_eq!(t.iter().collect_vec(), vec![0, 0, 1, 1, 1]);

        let mut t = empty_tree.clone();
        t.update(.., &1);
        assert_eq!(t.iter().collect_vec(), vec![1, 1, 1, 1, 1]);
    }

    // Query multiplies each value on a segment. Get computes the sum of values in a segment.
    #[test]
    fn heterogenous_tree_sum_of_products() {
        let mut t = SegmentTree::new(
            &vec![1; 10],
            0,
            1,
            |v1, v2| v1 + v2,
            |u1, u2| u1 * u2,
            |v, u, _, _| v * u,
        );
        t.update(1..=4, &10);
        t.update(3..=6, &2);
        assert_eq!(t.iter().collect_vec(), vec![1, 10, 10, 20, 20, 2, 2, 1, 1, 1]);
    }

    // Query applies a linear function on a segment. Get computes the sum of values in a segment.
    #[test]
    fn heterogenous_tree_sum_linear_functions() {
        // Adds (a + bi) to the i-th element.
        #[derive(Clone, Copy, Debug)]
        struct Update {
            a: i64,
            b: i64,
        }

        let mut t = SegmentTree::new(
            &vec![0; 10],
            0,
            Update { a: 0, b: 0 },
            |v1, v2| {
                v1 + v2
            },
            |u1, u2| {
                Update {
                    a: u1.a + u2.a,
                    b: u1.b + u2.b,
                }
            },
            |v, u, l, r| {
                let l = l as i64;
                let r = r as i64;
                v + u.a * (r - l) + u.b * (r*(r-1) - l*(l-1)) / 2
            },
        );
        t.update(1..6, &Update { a: 10, b: 2 });
        assert_eq!(t.get(0), 0);
        assert_eq!(t.get(1), 12);
        assert_eq!(t.get(2..=4), 48);
        assert_eq!(t.iter().collect_vec(), vec![0, 12, 14, 16, 18, 20, 0, 0, 0, 0]);
    }

    // Improvement potential. Find a less contrived example of a tree with no neutral update.
    #[test]
    fn optional_update() {
        type Update = Option<i64>;
        let mut t = SegmentTree::new(
            &vec![0; 10],
            0,
            None,
            |v1, v2| {
                v1 + v2
            },
            |u1: &Update, u2: &Update| {
                match (u1, u2) {
                    (Some(u1), Some(u2)) => Some(u1 + u2),
                    (Some(u), None) | (None, Some(u)) => Some(*u),
                    (None, None) => None,
                }
            },
            |v, u, l, r| {
                let Some(u) = u else {
                    return *v;
                };
                v + u * ((r - l) as i64)
            },
        );
        t.update(1..6, &Some(10));
        assert_eq!(t.iter().collect_vec(), vec![0, 10, 10, 10, 10, 10, 0, 0, 0, 0]);
    }
}
