use std::collections::{HashMap, HashSet};

use crate::graph::{Graph, VertexId};
use crate::io;


// TODO: Don't waste space on edge payloads when they are empty.
//   Consider having two separate neighbour mappings: one with payloads (when first vertex index is
//   smaller) and the other one without (for the other direction).
//   Or simply omit the `edges` map when `EP` is `()`.
#[derive(Clone, Debug)]
pub struct UndirectedGraph<VP, EP> {
    vertices: Vec<VP>,
    edges: HashMap<UndirectedEdgeId, EP>,
    neighbours: Vec<HashSet<VertexId>>,
}

impl<VP, EP> UndirectedGraph<VP, EP> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: HashMap::new(),
            neighbours: Vec::new(),
        }
    }

    pub fn add_vertex_p(&mut self, payload: VP) -> VertexId {
        let id = VertexId::from_0_based(self.vertices.len());
        self.vertices.push(payload);
        self.neighbours.push(HashSet::new());
        id
    }

    // Inserts an edge. Overwrites previous edge, if any.
    pub fn add_edge_p(&mut self, from: VertexId, to: VertexId, payload: EP) {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.insert(id, payload);
        self.neighbours[from].insert(to);
        self.neighbours[to].insert(from);
    }

    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<EP> {
        let id = UndirectedEdgeId::new(from, to);
        let payload = self.edges.remove(&id);
        self.neighbours[from].remove(&to);
        self.neighbours[to].remove(&from);
        payload
    }

    fn get_payload(&self, from: VertexId, to: VertexId) -> Option<&EP> {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.get(&id)
    }
}

impl<EP> UndirectedGraph<(), EP> {
    pub fn add_vertex(&mut self) -> VertexId {
        self.add_vertex_p(())
    }
    pub fn add_vertex_array<const N: usize>(&mut self) -> [VertexId; N] {
        [(); N].map(|()| self.add_vertex())
    }
    pub fn add_vertices(&mut self, num: usize) {
        for _ in 0..num {
            self.add_vertex();
        }
    }
    pub fn fit_vertex(&mut self, v: VertexId) {
        while self.vertices.len() <= v.to_0_based() as usize {
            self.add_vertex();
        }
    }
}

impl<VP> UndirectedGraph<VP, ()> {
    pub fn add_edge(&mut self, from: VertexId, to: VertexId) {
        self.add_edge_p(from, to, ());
    }
}

impl UndirectedGraph<(), ()> {
    // Reads edges as 1-based vertex pairs.
    pub fn from_read_edges<R: std::io::BufRead>(
        num_vertices: usize, num_edges: usize, read: &mut io::Reader<R>
    ) -> Self {
        let mut graph = Self::new();
        graph.add_vertices(num_vertices);
        for _ in 0..num_edges {
            let from = read.u32();
            let to = read.u32();
            graph.add_edge(VertexId::from_1_based(from), VertexId::from_1_based(to));
        }
        graph
    }
}

impl<VP, EP> Graph<VP, EP> for UndirectedGraph<VP, EP> {
    type VertexIter = Box<dyn Iterator<Item = VertexId>>;
    type HalfEdgeIter<'g> = Box<dyn Iterator<Item = (VertexId, &'g EP)> + 'g> where Self: 'g, EP: 'g;
    type FullEdgeIter<'g> = Box<dyn Iterator<Item = (VertexId, VertexId, &'g EP)> + 'g> where Self: 'g, EP: 'g;

    fn num_vertices(&self) -> usize { self.vertices.len() }

    fn vertex_ids(&self) -> Self::VertexIter {
        Box::new((0..self.vertices.len()).map(|i| VertexId::from_0_based(i)))
    }

    fn edges(&self) -> Self::FullEdgeIter<'_> {
        Box::new(self.edges.iter().map(|(e, payload)| (e.from, e.to, payload)))
    }

    fn vertex(&self, v: VertexId) -> &VP { &self.vertices[v] }
    fn vertex_mut(&mut self, v: VertexId) -> &mut VP { &mut self.vertices[v] }

    fn edge(&self, from: VertexId, to: VertexId) -> Option<&EP> {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.get(&id)
    }
    fn edge_mut(&mut self, from: VertexId, to: VertexId) -> Option<&mut EP> {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.get_mut(&id)
    }

    fn degree(&self, v: VertexId) -> u32 { self.neighbours[v].len() as u32 }
    fn out_degree(&self, v: VertexId) -> u32 { self.degree(v) }
    fn in_degree(&self, v: VertexId) -> u32 { self.degree(v) }

    fn edges_adj(&self, v: VertexId) -> Self::HalfEdgeIter<'_> {
        Box::new(self.neighbours[v].iter().map(move |&u| (u, self.get_payload(v, u).unwrap())))
    }
    fn edges_in(&self, to: VertexId) -> Self::HalfEdgeIter<'_> { self.edges_adj(to) }
    fn edges_out(&self, from: VertexId) -> Self::HalfEdgeIter<'_> { self.edges_adj(from) }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct UndirectedEdgeId {
    pub from: VertexId,
    pub to: VertexId,
}

impl UndirectedEdgeId {
    fn new(from: VertexId, to: VertexId) -> Self {
        let (from, to) = if from < to { (from, to) } else { (to, from) };
        Self { from, to }
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use crate::testing::io_utils::reader_from_string;

    use super::*;

    #[test]
    fn read_graph() {
        let input = "\
            4
            3
            1 3
            3 2
            3 4
        ";
        let mut read = reader_from_string(input);
        let n = read.usize();
        let m = read.usize();
        let g = UndirectedGraph::from_read_edges(n, m, &mut read);
        assert_eq!(g.num_vertices(), 4);
        let v1 = VertexId::from_1_based(1);
        let v2 = VertexId::from_1_based(2);
        let v3 = VertexId::from_1_based(3);
        let v4 = VertexId::from_1_based(4);
        assert!(g.edge(v1, v3).is_some());
        assert!(g.edge(v3, v2).is_some());
        assert!(g.edge(v3, v4).is_some());
        assert!(g.edge(v1, v2).is_none());
    }

    #[test]
    fn loops() {
        let mut g = UndirectedGraph::new();
        let [v1, v2, v3] = g.add_vertex_array();
        g.add_edge(v1, v1);
        g.add_edge(v1, v2);
        g.add_edge(v3, v3);
        assert_eq!(g.degree(v1), 2);
        assert_eq!(g.degree(v2), 1);
        assert_eq!(g.degree(v3), 1);
        assert_eq!(g.edges_adj(v1).map(|(w, _)| w).sorted().collect_vec(), vec![v1, v2]);
    }
}
