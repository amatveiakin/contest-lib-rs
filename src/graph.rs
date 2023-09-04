// TODO: See if graph module family can be simplified when impl specialization is stable (e.g the
// payload / no payload distinction):
//   - https://rust-lang.github.io/rfcs/1210-impl-specialization.html
//   - https://github.com/rust-lang/rust/issues/31844

// TODO: Replace
//     type VertexIter = Box<dyn Iterator<...>>;
// with
//     type VertexIter = impl Iterator<...>;
// in `Graph` implementations when "Permit impl Trait in type aliases" is stable:
//   - https://github.com/rust-lang/rust/issues/63063

// Improvement potential: Consider extending `VP == ()` and `EP == ()` specializations to
// `VP: Default` and `EP: Default`.

use std::fmt;
use std::ops;


// Use `u32` rather than `usize` to reduce memory used for edge storage.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VertexId { index: u32 }

// `Graph` is the common interface between different kinds of graphs (directed and undirected),
// graph algorithms and the end user.
//
// Features ([v] supported, [-] not supported):
//   [v] Directed and undirected graphs.
//   [v] Vertex and edge payloads.
//   [v] Loops.
//   [-] Parallel edges.
//   [v] Edge addition and removal.
//   [v] Vertex addition.
//   [-] Vertex removal.
//
// `Graph` trait deliberately omits the the methods for adding and removing edges in order to avoid
// confusion. These methods can be similar syntactically (in fact, `DirectedGraph` and
// `UndirectedGraph` can often be used interchangeably), but they have different semantics. For
// example, in an undirected graph the sequence
// ```
//     graph.add_edge(a, b);
//     graph.remove_edge(b, a);
// ```
// will return the graph to the original state (assuming that the edge didn't exist beforehand), and
// in a directed graph the edge (a, b) will remain alive. If you need to mutate a graph you almost
// certainly need to take either a `&mut DirectedGraph` or a `&mut UndirectedGraph`.
//
pub trait Graph<VP, EP> {
    type VertexIter: Iterator<Item = VertexId>;
    type HalfEdgeIter<'g>: Iterator<Item = (VertexId, &'g EP)> where Self: 'g, EP: 'g;

    fn num_vertices(&self) -> usize;

    // Vertex IDs always range from 0 to (num_vertices() - 1).
    fn vertex_ids(&self) -> Self::VertexIter;

    fn vertex(&self, v: VertexId) -> &VP;
    fn vertex_mut(&mut self, v: VertexId) -> &mut VP;

    fn edge(&self, from: VertexId, to: VertexId) -> Option<&EP>;
    fn edge_mut(&mut self, from: VertexId, to: VertexId) -> Option<&mut EP>;

    // For directed graphs: `degree` == `in_degree` + `out_degree`.
    // For undirected graphs: `degree` == `in_degree` == `out_degree`.
    // Note. The invariants above always hold, which means that loops are counted twice in directed
    // graphs and once in undirected graphs.
    fn degree(&self, v: VertexId) -> u32;
    fn out_degree(&self, v: VertexId) -> u32;
    fn in_degree(&self, v: VertexId) -> u32;

    // Guarantees:
    //   - `edges_in().count()` == `in_degree()`;
    //   - `edges_out().count()` == `out_degree()`;
    // Iteration order is unspecified. (Note. It's easy to fix it if necessary by replacing
    // `HashSet` with `BTreeSet`.)
    fn edges_in(&self, to: VertexId) -> Self::HalfEdgeIter<'_>;
    fn edges_out(&self, from: VertexId) -> Self::HalfEdgeIter<'_>;
}

impl VertexId {
    pub fn from_0_based<T>(index: T) -> Self
    where
        T: TryInto<u32>,
        <T as TryInto<u32>>::Error: fmt::Debug,
    {
        Self { index: index.try_into().unwrap() }
    }
    pub fn from_1_based<T>(index: T) -> Self
    where
        T: TryInto<u32>,
        <T as TryInto<u32>>::Error: fmt::Debug,
    {
        Self { index: index.try_into().unwrap().checked_sub(1).unwrap() }
    }
    pub fn to_0_based(&self) -> u32 { self.index }
    pub fn to_1_based(&self) -> u32 { self.index + 1 }
}

// For output convenience.
pub trait VertexIdVec {
    fn to_0_based_vec(&self) -> Vec<u32>;
    fn to_1_based_vec(&self) -> Vec<u32>;
}
impl VertexIdVec for Vec<VertexId> {
    fn to_0_based_vec(&self) -> Vec<u32> { self.iter().map(|v| v.to_0_based()).collect() }
    fn to_1_based_vec(&self) -> Vec<u32> { self.iter().map(|v| v.to_1_based()).collect() }
}

impl<T> ops::Index<VertexId> for [T] {
    type Output = T;
    fn index(&self, v: VertexId) -> &T { &self[v.to_0_based() as usize] }
}
impl<T> ops::IndexMut<VertexId> for [T] {
    fn index_mut(&mut self, v: VertexId) -> &mut T { &mut self[v.to_0_based() as usize] }
}

impl<T> ops::Index<VertexId> for Vec<T> {
    type Output = T;
    fn index(&self, v: VertexId) -> &T { &self[v.to_0_based() as usize] }
}
impl<T> ops::IndexMut<VertexId> for Vec<T> {
    fn index_mut(&mut self, v: VertexId) -> &mut T { &mut self[v.to_0_based() as usize] }
}
