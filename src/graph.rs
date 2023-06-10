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

use std::ops;


// Use `u32` rather than `usize` to reduce memory used for edge storage.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VertexId { index: u32 }

// TODO: Consider cloning the payload instead.
// Edge, as viewed from one vertex (which could be either the head or the tail).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HalfEdge<'g, EP> {
    pub other: VertexId,
    pub payload: &'g EP,
}

// `Graph` is the common interface between different kinds of graphs (directed and undirected),
// graph algorithms and the end user.
//
// Features ([v] supported, [-] not supported):
//   [v] Directed and undirected graphs.
//   [v] Vertex and edge payloads.
//   [-] Parallel edges and loops.
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
pub trait Graph<'g, VP, EP: 'g> {
    type VertexIter: Iterator<Item = VertexId>;
    type HalfEdgeIter: Iterator<Item = HalfEdge<'g, EP>>;

    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;

    // Vertex IDs always range from 0 to (num_vertices() - 1).
    fn vertex_ids(&self) -> Self::VertexIter;

    fn vertex(&'g self, v: VertexId) -> &'g VP;
    fn vertex_mut(&'g mut self, v: VertexId) -> &'g mut VP;

    fn edge(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP>;
    fn edge_mut(&'g mut self, from: VertexId, to: VertexId) -> Option<&'g mut EP>;

    // For directed graphs: `degree` == `in_degree` + `out_degree`.
    // For undirected graphs: `degree` == `in_degree` == `out_degree`.
    fn degree(&'g self, v: VertexId) -> u32;
    fn out_degree(&'g self, v: VertexId) -> u32;
    fn in_degree(&'g self, v: VertexId) -> u32;

    // Guarantees:
    //   - `edges_in().count()` == `in_degree()`;
    //   - `edges_out().count()` == `out_degree()`;
    // Iteration order is unspecified. (Note. It's easy to fix it if necessary by replacing
    // `HashSet` with `BTreeSet`.)
    fn edges_in(&'g self, to: VertexId) -> Self::HalfEdgeIter;
    fn edges_out(&'g self, from: VertexId) -> Self::HalfEdgeIter;
}

impl VertexId {
    pub fn from_0_based(index: u32) -> Self { Self { index } }
    pub fn from_1_based(index: u32) -> Self { Self { index: index.checked_sub(1).unwrap() } }
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
