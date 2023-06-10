use std::collections::{HashMap, HashSet};

use crate::graph::{Graph, VertexId, HalfEdge};
use crate::io;


#[derive(Clone, Debug)]
pub struct DirectedGraph<VP, EP> {
    vertices: Vec<VP>,
    edges_out: Vec<HashMap<VertexId, EP>>,
    edges_in: Vec<HashSet<VertexId>>,
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
        let id = VertexId::from_0_based(self.vertices.len().try_into().unwrap());
        self.vertices.push(payload);
        self.edges_out.push(HashMap::new());
        self.edges_in.push(HashSet::new());
        id
    }

    // Inserts an edge. Overwrites previous edge, if any.
    pub fn add_edge_p(&mut self, from: VertexId, to: VertexId, payload: EP) {
        self.edges_out[from].insert(to, payload);
        self.edges_in[to].insert(from);
    }

    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<EP> {
        let payload = self.edges_out[from].remove(&to);
        self.edges_in[to].remove(&from);
        payload
    }
}

impl<EP> DirectedGraph<(), EP> {
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

impl<VP> DirectedGraph<VP, ()> {
    pub fn add_edge(&mut self, from: VertexId, to: VertexId) {
        self.add_edge_p(from, to, ());
    }
}

impl DirectedGraph<(), ()> {
    // Reads number of vertices, then number of edges, then edges as 1-based vertex pairs.
    pub fn from_edges<R: std::io::BufRead>(read: &mut io::Reader<R>) -> Self {
        let n = read.usize();
        let m = read.usize();
        let mut graph = Self::new();
        graph.add_vertices(n);
        for _ in 0..m {
            let from = read.u32();
            let to = read.u32();
            graph.add_edge(VertexId::from_1_based(from), VertexId::from_1_based(to));
        }
        graph
    }
}

impl<'g, VP, EP: 'g> Graph<'g, VP, EP> for DirectedGraph<VP, EP> {
    type VertexIter = Box<dyn Iterator<Item = VertexId>>;
    type HalfEdgeIter = Box<dyn Iterator<Item = HalfEdge<'g, EP>> + 'g>;

    fn num_vertices(&self) -> usize { self.vertices.len() }

    fn vertex_ids(&self) -> Self::VertexIter {
        Box::new((0..self.vertices.len()).map(|i| VertexId::from_0_based(i.try_into().unwrap())))
    }

    fn vertex(&'g self, v: VertexId) -> &'g VP { &self.vertices[v] }
    fn vertex_mut(&'g mut self, v: VertexId) -> &'g mut VP { &mut self.vertices[v] }

    fn edge(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP> {
        self.edges_out[from].get(&to)
    }
    fn edge_mut(&'g mut self, from: VertexId, to: VertexId) -> Option<&'g mut EP> {
        self.edges_out[from].get_mut(&to)
    }

    fn degree(&'g self, v: VertexId) -> u32 { self.in_degree(v) + self.out_degree(v) }
    fn out_degree(&'g self, v: VertexId) -> u32 { self.edges_out[v].len() as u32 }
    fn in_degree(&'g self, v: VertexId) -> u32 { self.edges_in[v].len() as u32 }

    fn edges_in(&'g self, to: VertexId) -> Self::HalfEdgeIter {
        Box::new(self.edges_in[to]
            .iter()
            .map(move |&from| HalfEdge { other: from, payload: self.edge(from, to).unwrap() }))
    }
    fn edges_out(&'g self, from: VertexId) -> Self::HalfEdgeIter {
        Box::new(self.edges_out[from]
            .iter()
            .map(move |(&to, payload)| HalfEdge { other: to, payload }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn read_graph() {
        let input = "\
            4
            3
            1 3
            3 2
            3 4
        ";
        let mut read = io::Reader::new(std::io::Cursor::new(input.to_owned().into_bytes()));
        let g = DirectedGraph::from_edges(&mut read);
        assert_eq!(g.num_vertices(), 4);
        let v1 = VertexId::from_1_based(1);
        let v2 = VertexId::from_1_based(2);
        let v3 = VertexId::from_1_based(3);
        let v4 = VertexId::from_1_based(4);
        assert!(g.edge(v1, v3).is_some());
        assert!(g.edge(v3, v2).is_some());
        assert!(g.edge(v3, v4).is_some());
        assert!(g.edge(v1, v2).is_none());
        assert!(g.edge(v3, v1).is_none());
    }
}
