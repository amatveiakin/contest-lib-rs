use crate::directed_graph::DirectedGraph;
use crate::graph::VertexId;
use crate::undirected_graph::UndirectedGraph;


pub trait UndirectedGraphExtension {
    // TODO: Move `add_vertex_array` here.
    fn add_chain<const N: usize>(&mut self) -> [VertexId; N];
    fn add_cycle<const N: usize>(&mut self) -> [VertexId; N];
    fn add_clique<const N: usize>(&mut self) -> [VertexId; N];
}

impl UndirectedGraphExtension for UndirectedGraph<(), ()> {
    fn add_chain<const N: usize>(&mut self) -> [VertexId; N] {
        assert!(N >= 2);
        let vertices = self.add_vertex_array();
        for i in 0..N-1 {
            self.add_edge(vertices[i], vertices[i + 1]);
        }
        vertices
    }

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


pub trait DirectedGraphExtension {
    // TODO: Move `add_vertex_array` here.
    fn add_unidirectional_chain<const N: usize>(&mut self) -> [VertexId; N];
    fn bidirectional_chain<const N: usize>(&mut self) -> [VertexId; N];
    fn add_unidirectional_cycle<const N: usize>(&mut self) -> [VertexId; N];
    fn add_bidirectional_cycle<const N: usize>(&mut self) -> [VertexId; N];
    fn add_full_clique<const N: usize>(&mut self) -> [VertexId; N];
    fn add_dag_clique<const N: usize>(&mut self) -> [VertexId; N];
}

impl DirectedGraphExtension for DirectedGraph<(), ()> {
    fn add_unidirectional_chain<const N: usize>(&mut self) -> [VertexId; N] {
        assert!(N >= 2);
        let vertices = self.add_vertex_array();
        for i in 0..N-1 {
            self.add_edge(vertices[i], vertices[i + 1]);
        }
        vertices
    }

    fn bidirectional_chain<const N: usize>(&mut self) -> [VertexId; N] {
        assert!(N >= 2);
        let vertices = self.add_vertex_array();
        for i in 0..N-1 {
            self.add_edge(vertices[i], vertices[i + 1]);
        }
        vertices
    }

    fn add_unidirectional_cycle<const N: usize>(&mut self) -> [VertexId; N] {
        assert!(N >= 3);
        let vertices = self.add_vertex_array();
        for i in 0..N {
            self.add_edge(vertices[i], vertices[(i + 1) % N]);
        }
        vertices
    }

    fn add_bidirectional_cycle<const N: usize>(&mut self) -> [VertexId; N] {
        assert!(N >= 3);
        let vertices = self.add_vertex_array();
        for i in 0..N {
            self.add_edge(vertices[i], vertices[(i + 1) % N]);
            self.add_edge(vertices[(i + 1) % N], vertices[i]);
        }
        vertices
    }

    fn add_full_clique<const N: usize>(&mut self) -> [VertexId; N] {
        let vertices = self.add_vertex_array();
        for i in 0..N {
            for j in i+1..N {
                self.add_edge(vertices[i], vertices[j]);
                self.add_edge(vertices[j], vertices[i]);
            }
        }
        vertices
    }

    fn add_dag_clique<const N: usize>(&mut self) -> [VertexId; N] {
        let vertices = self.add_vertex_array();
        for i in 0..N {
            for j in i+1..N {
                self.add_edge(vertices[i], vertices[j]);
            }
        }
        vertices
    }
}
