use std::array;
use std::collections::{HashMap, HashSet};

use crate::base_one::BaseOneConversion;
use crate::graph::{Graph, VertexId, StorageVertexId};
use crate::io;


// TODO: Don't waste space on edge payloads when they are empty.
//   Consider having two separate neighbour mappings: one with payloads (when first vertex index is
//   smaller) and the other one without (for the other direction).
//   Or simply omit the `edges` map when `EP` is `()`.
#[derive(Clone, Debug)]
pub struct UndirectedGraph<VP, EP> {
    vertices: Vec<VP>,
    edges: HashMap<UndirectedEdgeId, EP>,
    neighbours: Vec<HashSet<StorageVertexId>>,
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
        let id = self.vertices.len();
        self.vertices.push(payload);
        self.neighbours.push(HashSet::new());
        id
    }

    // Inserts an edge. Overwrites previous edge, if any.
    pub fn add_edge_p(&mut self, from: VertexId, to: VertexId, payload: EP) {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.insert(id, payload);
        self.neighbours[from].insert(to as StorageVertexId);
        self.neighbours[to].insert(from as StorageVertexId);
    }

    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<EP> {
        let id = UndirectedEdgeId::new(from, to);
        let payload = self.edges.remove(&id);
        self.neighbours[from].remove(&(to as StorageVertexId));
        self.neighbours[to].remove(&(from as StorageVertexId));
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
        array::from_fn(|_| self.add_vertex())
    }
    pub fn add_vertices(&mut self, num: usize) {
        for _ in 0..num {
            self.add_vertex();
        }
    }
    pub fn fit_vertex(&mut self, v: VertexId) {
        while self.vertices.len() <= v {
            self.add_vertex();
        }
    }

    pub fn from_read_edges_p<R: std::io::BufRead>(
        num_vertices: usize, num_edges: usize, read: &mut io::Reader<R>,
        read_payload: impl Fn(&mut io::Reader<R>) -> EP
    ) -> Self {
        let mut graph = Self::new();
        graph.add_vertices(num_vertices);
        for _ in 0..num_edges {
            let [from, to] = read.usizes().from1b();
            let payload = read_payload(read);
            graph.add_edge_p(from, to, payload);
        }
        graph
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
        Self::from_read_edges_p(num_vertices, num_edges, read, |_| ())
    }
}

impl<VP, EP> Graph<VP, EP> for UndirectedGraph<VP, EP> {
    const IS_DIRECTED: bool = false;

    fn num_vertices(&self) -> usize { self.vertices.len() }
    fn num_edges(&self) -> usize { self.edges.len() }

    fn edges<'g>(&'g self) -> impl Iterator<Item = (VertexId, VertexId, &'g EP)> where EP: 'g {
        self.edges.iter().map(|(e, payload)| (e.from as VertexId, e.to as VertexId, payload))
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

    fn degree(&self, v: VertexId) -> usize { self.neighbours[v].len() }
    fn out_degree(&self, v: VertexId) -> usize { self.degree(v) }
    fn in_degree(&self, v: VertexId) -> usize { self.degree(v) }

    fn edges_adj<'g>(&'g self, v: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.neighbours[v].iter().map(
            move |&u| (u as VertexId, self.get_payload(v, u as VertexId).unwrap()))
    }
    fn edges_in<'g>(&'g self, to: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_adj(to)
    }
    fn edges_out<'g>(&'g self, from: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_adj(from)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct UndirectedEdgeId {
    pub from: StorageVertexId,
    pub to: StorageVertexId,
}

impl UndirectedEdgeId {
    fn new(from: VertexId, to: VertexId) -> Self {
        let from = from as StorageVertexId;
        let to = to as StorageVertexId;
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
        let [v1, v2, v3, v4] = [1, 2, 3, 4].from1b();
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
