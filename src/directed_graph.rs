use std::array;
use std::collections::{HashMap, HashSet};

use crate::base_one::{Base, BaseOneConversion};
use crate::graph::{Graph, VertexId, StorageVertexId};
use crate::io;


#[derive(Clone, Debug)]
pub struct DirectedGraph<VP, EP> {
    vertices: Vec<VP>,
    edges_out: Vec<HashMap<StorageVertexId, EP>>,
    edges_in: Vec<HashSet<StorageVertexId>>,
}

impl<VP, EP> DirectedGraph<VP, EP> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges_out: Vec::new(),
            edges_in: Vec::new(),
        }
    }

    pub fn add_vertex_p(&mut self, payload: VP) -> VertexId {
        let id = self.vertices.len();
        self.vertices.push(payload);
        self.edges_out.push(HashMap::new());
        self.edges_in.push(HashSet::new());
        id
    }

    // Inserts an edge. Overwrites previous edge, if any.
    pub fn add_edge_p(&mut self, from: VertexId, to: VertexId, payload: EP) {
        self.edges_out[from].insert(to as StorageVertexId, payload);
        self.edges_in[to].insert(from as StorageVertexId);
    }

    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<EP> {
        let payload = self.edges_out[from].remove(&(to as StorageVertexId));
        self.edges_in[to].remove(&(from as StorageVertexId));
        payload
    }
}

impl<EP> DirectedGraph<(), EP> {
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
        num_vertices: usize, num_edges: usize, base: Base, read: &mut io::Reader<R>,
        read_payload: impl Fn(&mut io::Reader<R>) -> EP
    ) -> Self {
        let mut graph = Self::new();
        graph.add_vertices(num_vertices);
        for _ in 0..num_edges {
            let [from, to] = read.usizes().from_base(base);
            let payload = read_payload(read);
            graph.add_edge_p(from, to, payload);
        }
        graph
    }
}

impl<VP> DirectedGraph<VP, ()> {
    pub fn add_edge(&mut self, from: VertexId, to: VertexId) {
        self.add_edge_p(from, to, ());
    }
}

impl DirectedGraph<(), ()> {
    // Reads edges as 1-based vertex pairs.
    pub fn from_read_edges<R: std::io::BufRead>(
        num_vertices: usize, num_edges: usize, base: Base, read: &mut io::Reader<R>
    ) -> Self {
        Self::from_read_edges_p(num_vertices, num_edges, base, read, |_| ())
    }
}

impl<VP, EP> Graph<VP, EP> for DirectedGraph<VP, EP> {
    const IS_DIRECTED: bool = true;

    fn num_vertices(&self) -> usize { self.vertices.len() }
    fn num_edges(&self) -> usize {
        self.edges_out.iter().map(|edges_out| edges_out.len()).sum()
    }

    fn edges<'g>(&'g self) -> impl Iterator<Item = (VertexId, VertexId, &'g EP)> where EP: 'g {
        self.edges_out.iter().enumerate().flat_map(move |(from, edges_out)| {
            edges_out.iter().map(move |(to, payload)| (from, *to as VertexId, payload))
        })
    }

    fn vertex(&self, v: VertexId) -> &VP { &self.vertices[v] }
    fn vertex_mut(&mut self, v: VertexId) -> &mut VP { &mut self.vertices[v] }

    fn edge(&self, from: VertexId, to: VertexId) -> Option<&EP> {
        self.edges_out[from].get(&(to as StorageVertexId))
    }
    fn edge_mut(&mut self, from: VertexId, to: VertexId) -> Option<&mut EP> {
        self.edges_out[from].get_mut(&(to as StorageVertexId))
    }

    fn degree(&self, v: VertexId) -> usize { self.in_degree(v) + self.out_degree(v) }
    fn out_degree(&self, v: VertexId) -> usize { self.edges_out[v].len() }
    fn in_degree(&self, v: VertexId) -> usize { self.edges_in[v].len() }

    fn edges_adj<'g>(&'g self, v: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_out(v).chain(self.edges_in(v))
    }
    fn edges_in<'g>(&'g self, to: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_in[to]
            .iter()
            .map(move |&from| (from as VertexId, self.edge(from as VertexId, to).unwrap()))
    }
    fn edges_out<'g>(&'g self, from: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_out[from]
            .iter()
            .map(move |(&to, payload)| (to as VertexId, payload))
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use crate::base_one::Base;
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
        let g = DirectedGraph::from_read_edges(n, m, Base::ONE, &mut read);
        assert_eq!(g.num_vertices(), 4);
        let [v1, v2, v3, v4] = [1, 2, 3, 4].from1b();
        assert!(g.edge(v1, v3).is_some());
        assert!(g.edge(v3, v2).is_some());
        assert!(g.edge(v3, v4).is_some());
        assert!(g.edge(v1, v2).is_none());
        assert!(g.edge(v3, v1).is_none());
    }

    #[test]
    fn loops() {
        let mut g = DirectedGraph::new();
        let [v1, v2, v3] = g.add_vertex_array();
        g.add_edge(v1, v1);
        g.add_edge(v1, v2);
        g.add_edge(v3, v3);
        assert_eq!(g.degree(v1), 3);
        assert_eq!(g.degree(v2), 1);
        assert_eq!(g.degree(v3), 2);
        assert_eq!(g.edges_out(v1).map(|(u, _)| u).sorted().collect_vec(), vec![v1, v2]);
        assert_eq!(g.edges_in(v1).map(|(u, _)| u).sorted().collect_vec(), vec![v1]);
    }
}
