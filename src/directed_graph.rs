use std::collections::{HashMap, HashSet};

use crate::graph::{Graph, VertexId, HalfEdge};


#[derive(Clone, Debug)]
pub struct DirectedGraph<VP, EP> {
    vertices: Vec<VP>,
    edges: HashMap<DirectedEdgeId, EP>,
    edges_out: Vec<HashSet<VertexId>>,
    edges_in: Vec<HashSet<VertexId>>,
}

impl<VP, EP> DirectedGraph<VP, EP> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: HashMap::new(),
            edges_out: Vec::new(),
            edges_in: Vec::new(),
        }
    }

    pub fn add_vertex_p(&mut self, payload: VP) -> VertexId {
        let id = VertexId::from_0_based(self.vertices.len().try_into().unwrap());
        self.vertices.push(payload);
        self.edges_out.push(HashSet::new());
        self.edges_in.push(HashSet::new());
        id
    }

    // Inserts an edge. Overwrites previous edge, if any.
    pub fn add_edge_p(&mut self, from: VertexId, to: VertexId, payload: EP) {
        let id = DirectedEdgeId::new(from, to);
        self.edges.insert(id, payload);
        self.edges_out[from.to_0_based() as usize].insert(to);
        self.edges_in[to.to_0_based() as usize].insert(from);
    }

    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<EP> {
        let id = DirectedEdgeId::new(from, to);
        let payload = self.edges.remove(&id);
        self.edges_out[from.to_0_based() as usize].remove(&to);
        self.edges_in[to.to_0_based() as usize].remove(&from);
        payload
    }

    fn get_payload<'g>(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP> {
        let id = DirectedEdgeId::new(from, to);
        self.edges.get(&id)
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

impl<'g, VP, EP: 'g> Graph<'g, VP, EP> for DirectedGraph<VP, EP> {
    type VertexIter = Box<dyn Iterator<Item = VertexId>>;
    type HalfEdgeIter = Box<dyn Iterator<Item = HalfEdge<'g, EP>> + 'g>;

    fn num_vertices(&self) -> usize { self.vertices.len() }
    fn num_edges(&self) -> usize { self.edges.len() }

    fn vertex_ids(&self) -> Self::VertexIter {
        Box::new((0..self.vertices.len()).map(|i| VertexId::from_0_based(i.try_into().unwrap())))
    }

    fn vertex(&'g self, v: VertexId) -> &'g VP {
        &self.vertices[v.to_0_based() as usize]
    }
    fn vertex_mut(&'g mut self, v: VertexId) -> &'g mut VP {
        &mut self.vertices[v.to_0_based() as usize]
    }

    fn edge(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP> {
        let id = DirectedEdgeId::new(from, to);
        self.edges.get(&id)
    }
    fn edge_mut(&'g mut self, from: VertexId, to: VertexId) -> Option<&'g mut EP> {
        let id = DirectedEdgeId::new(from, to);
        self.edges.get_mut(&id)
    }

    fn degree(&'g self, v: VertexId) -> u32 {
        self.in_degree(v) + self.out_degree(v)
    }
    fn out_degree(&'g self, v: VertexId) -> u32 {
        self.edges_out[v.to_0_based() as usize].len() as u32
    }
    fn in_degree(&'g self, v: VertexId) -> u32 {
        self.edges_in[v.to_0_based() as usize].len() as u32
    }

    fn edges_in(&'g self, to: VertexId) -> Self::HalfEdgeIter {
        Box::new(self.edges_in[to.to_0_based() as usize]
            .iter()
            .map(move |&from| HalfEdge { other: from, payload: self.get_payload(from, to).unwrap() }))
    }
    fn edges_out(&'g self, from: VertexId) -> Self::HalfEdgeIter {
        Box::new(self.edges_out[from.to_0_based() as usize]
            .iter()
            .map(move |&to| HalfEdge { other: to, payload: self.get_payload(from, to).unwrap() }))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct DirectedEdgeId {
    pub from: VertexId,
    pub to: VertexId,
}

impl DirectedEdgeId {
    pub fn new(from: VertexId, to: VertexId) -> Self { Self { from, to } }
}
