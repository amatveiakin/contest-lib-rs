use std::collections::{HashMap, HashSet};

use crate::graph::{Graph, VertexId, HalfEdge};


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
        let id = VertexId::from_0_based(self.vertices.len().try_into().unwrap());
        self.vertices.push(payload);
        self.neighbours.push(HashSet::new());
        id
    }

    // Inserts an edge. Overwrites previous edge, if any.
    pub fn add_edge_p(&mut self, from: VertexId, to: VertexId, payload: EP) {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.insert(id, payload);
        self.neighbours[from.to_0_based() as usize].insert(to);
        self.neighbours[to.to_0_based() as usize].insert(from);
    }

    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Option<EP> {
        let id = UndirectedEdgeId::new(from, to);
        let payload = self.edges.remove(&id);
        self.neighbours[from.to_0_based() as usize].remove(&to);
        self.neighbours[to.to_0_based() as usize].remove(&from);
        payload
    }

    pub fn adjacent(&self, v: VertexId) -> impl Iterator<Item = HalfEdge<'_, EP>> {
        self.neighbours[v.to_0_based() as usize]
            .iter()
            .map(move |&u| HalfEdge { other: u, payload: self.get_payload(v, u).unwrap() })
    }

    fn get_payload<'g>(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP> {
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

impl<'g, VP, EP: 'g> Graph<'g, VP, EP> for UndirectedGraph<VP, EP> {
    type EdgeIter = Box<dyn Iterator<Item = HalfEdge<'g, EP>> + 'g>;

    fn num_vertices(&self) -> usize { self.vertices.len() }
    fn num_edges(&self) -> usize { self.edges.len() }

    fn vertex(&'g self, v: VertexId) -> &'g VP {
        &self.vertices[v.to_0_based() as usize]
    }
    fn vertex_mut(&'g mut self, v: VertexId) -> &'g mut VP {
        &mut self.vertices[v.to_0_based() as usize]
    }

    fn edge(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP> {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.get(&id)
    }
    fn edge_mut(&'g mut self, from: VertexId, to: VertexId) -> Option<&'g mut EP> {
        let id = UndirectedEdgeId::new(from, to);
        self.edges.get_mut(&id)
    }

    fn degree(&'g self, v: VertexId) -> u32 {
        self.neighbours[v.to_0_based() as usize].len() as u32
    }
    fn out_degree(&'g self, v: VertexId) -> u32 { self.degree(v) }
    fn in_degree(&'g self, v: VertexId) -> u32 { self.degree(v) }

    fn edges_from(&'g self, from: VertexId) -> Self::EdgeIter { Box::new(self.adjacent(from)) }
    fn edges_to(&'g self, to: VertexId) -> Self::EdgeIter { Box::new(self.adjacent(to)) }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct UndirectedEdgeId {
    pub from: VertexId,
    pub to: VertexId,
}

impl UndirectedEdgeId {
    pub fn new(from: VertexId, to: VertexId) -> Self {
        let (from, to) = if from < to { (from, to) } else { (to, from) };
        Self { from, to }
    }
}
