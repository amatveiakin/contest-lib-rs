use crate::graph::VertexId;
use crate::undirected_graph::UndirectedGraph;


// TODO: Add `DirectedGraphExtension` (split `add_cycle` into unidirectional and bidirectional
// versions; split `add_clique` into bidirectional, DAG and random versions).

pub trait UndirectedGraphExtension {
    // TODO: Move `add_vertex_array` here.
    fn add_cycle<const N: usize>(&mut self) -> [VertexId; N];
    fn add_clique<const N: usize>(&mut self) -> [VertexId; N];
}

impl UndirectedGraphExtension for UndirectedGraph<(), ()> {
    fn add_cycle<const N: usize>(&mut self) -> [VertexId; N] {
        assert!(N >= 3);
        let vertices = self.add_vertex_array();
        for i in 0..N {
            self.add_edge(vertices[i], vertices[(i + 1) % N]);
        }
        vertices
    }

    fn add_clique<const N: usize>(&mut self) -> [VertexId; N] {
        let vertices = self.add_vertex_array();
        for i in 0..N {
            for j in i+1..N {
                self.add_edge(vertices[i], vertices[j]);
            }
        }
        vertices
    }
}
