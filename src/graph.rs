// TODO: See if graph module family can be simplified when impl specialization is stable (e.g the
// payload / no payload distinction):
//   - https://rust-lang.github.io/rfcs/1210-impl-specialization.html
//   - https://github.com/rust-lang/rust/issues/31844

// Improvement potential: Consider extending `VP == ()` and `EP == ()` specializations to
// `VP: Default` and `EP: Default`.


// Vertex IDs must fit in `StorageVertexId`! The latter is used internally by `Graph`
// implementations in order to save memory. Using `usize` as public interface to reduce the amount
// of type casting.
pub type VertexId = usize;
pub type StorageVertexId = u32;

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
// in a directed graph the edge (a, b) will remain alive. If you require to mutate graph structure,
// you almost certainly need to take either a `&mut DirectedGraph` or a `&mut UndirectedGraph`.
//
pub trait Graph<VP, EP> {
    const IS_DIRECTED: bool;
    fn is_directed(&self) -> bool { Self::IS_DIRECTED }

    // `num_vertices` is always O(1), but `num_edges` could be O(V).
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;

    // Vertex IDs always range from 0 to (num_vertices() - 1).
    fn vertex_ids(&self) -> VertexIdIterator { VertexIdIterator { current: 0, end: self.num_vertices() } }

    fn edges<'g>(&'g self) -> impl Iterator<Item = (VertexId, VertexId, &'g EP)> where EP: 'g;

    fn vertex(&self, v: VertexId) -> &VP;
    fn vertex_mut(&mut self, v: VertexId) -> &mut VP;

    fn edge(&self, from: VertexId, to: VertexId) -> Option<&EP>;
    fn edge_mut(&mut self, from: VertexId, to: VertexId) -> Option<&mut EP>;

    // For directed graphs: `degree` == `in_degree` + `out_degree`.
    // For undirected graphs: `degree` == `in_degree` == `out_degree`.
    // Note. The invariants above always hold, which means that loops are counted twice in directed
    // graphs and once in undirected graphs.
    fn degree(&self, v: VertexId) -> usize;
    fn out_degree(&self, v: VertexId) -> usize;
    fn in_degree(&self, v: VertexId) -> usize;

    // Guarantees:
    //   - `edges_adj().count()` == `degree()`;
    //   - `edges_in().count()` == `in_degree()`;
    //   - `edges_out().count()` == `out_degree()`;
    // Iteration order is unspecified. (Note. It's easy to fix it if necessary by replacing
    // `HashSet` with `BTreeSet`.)
    fn edges_adj<'g>(&'g self, from: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g;
    fn edges_in<'g>(&'g self, to: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g;
    fn edges_out<'g>(&'g self, from: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g;
}

// Rust-upgrade: replpace with
//   fn vertex_ids(&self) -> impl Iterator<Item = VertexId> { 0..self.vertices.len() }
// when there is a way to fix that fact that &self lifetime is captired. For now there isn't:
// https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html#overcapturing
// Capturing &self lifetime is problematic because it breaks code like
//   for v in g.vertex_ids() { ... g.add_edge(v, u); ... }
pub struct VertexIdIterator {
    current: VertexId,
    end: VertexId,
}

impl Iterator for VertexIdIterator {
    type Item = VertexId;
    fn next(&mut self) -> Option<VertexId> {
        if self.current < self.end {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}
