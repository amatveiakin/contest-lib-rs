// `UndirectedComboGraph` is an undirected graph that allows combining multiple vertices into a
// single one.
//
// Combining vertices any number of times in a graph with V vertices and E edges is done in
//   O((V + E) log V)
// time.
//
// The actual number of vertices remains unchanged. Instead, one vertex in a group is chosen as a
// `Pivot` and all other vertices become `Alias`es to it. All edges to/from aliases are reconnected
// to the pivot.

use std::ops::Deref;

use crate::graph::{Graph, VertexId};
use crate::undirected_graph::UndirectedGraph;


#[derive(Clone, Debug)]
pub enum ComboVertex {
    Pivot(Vec<VertexId>),  // vertices in the group
    Alias(VertexId),
}

#[derive(Clone, Debug)]
pub struct UndirectedComboGraph {
    graph: UndirectedGraph<(), ()>,
    combo_vertices: Vec<ComboVertex>,
}

impl UndirectedComboGraph {
    pub fn new(graph: UndirectedGraph<(), ()>) -> Self {
        let vertices = (0..graph.num_vertices()).map(|v| ComboVertex::Pivot(vec![v])).collect();
        Self { graph, combo_vertices: vertices }
    }

    pub fn into_graph(self) -> UndirectedGraph<(), ()> { self.graph }
    pub fn graph(&self) -> &UndirectedGraph<(), ()> { &self.graph }
    // Use carefully: do not add edges to `Alias` vertices.
    pub fn graph_mut(&mut self) -> &mut UndirectedGraph<(), ()> { &mut self.graph }

    pub fn combo_vertices(&self) -> &[ComboVertex] { &self.combo_vertices }

    // Combines several `Pivot` vertices and returns the new pivot.
    pub fn comine_vertices(&mut self, vertices: &[VertexId]) -> VertexId {
        let pivot = vertices.iter().copied().max_by_key(|&v| match self.combo_vertices[v] {
            // Most of the time, vertex degree would suffice. However, vertex group size needs to be
            // taken into account to avoid quadratic performance in case the of an edgeless graph.
            ComboVertex::Pivot(ref vs) => self.graph.degree(v) + vs.len(),
            ComboVertex::Alias(_) => panic!("Cannot combine `Alias` vertex"),
        }).unwrap();

        for &v in vertices {
            if v == pivot {
                continue;
            }
            self.combo_vertices[v] = ComboVertex::Alias(pivot);
            let ComboVertex::Pivot(ref mut group) = self.combo_vertices[pivot] else {
                unreachable!();
            };
            group.push(v);
            let nbrs = self.graph.edges_adj(v).map(|(u, _)| u).collect::<Vec<_>>();
            for u in nbrs {
                let payload = self.graph.remove_edge(v, u).unwrap();
                if u != pivot {
                    self.graph.add_edge_p(pivot, u, payload);
                }
            }
        }
        pivot
    }
}

impl Deref for UndirectedComboGraph {
    type Target = UndirectedGraph<(), ()>;
    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::testing::graph_ext::UndirectedGraphExtension;
    use crate::undirected_graph::UndirectedEdgeId;

    use super::*;

    fn to_edges(g: &UndirectedComboGraph) -> HashSet<UndirectedEdgeId> {
        g.edges().map(|(u, v, _)| UndirectedEdgeId::new(u, v)).collect()
    }

    #[test]
    fn fully_collapse_cycle() {
        let mut graph = UndirectedGraph::new();
        let [a, b, c, d] = graph.add_cycle();
        let mut cg = UndirectedComboGraph::new(graph);
        assert_eq!(to_edges(&cg), [
            UndirectedEdgeId::new(a, b),
            UndirectedEdgeId::new(b, c),
            UndirectedEdgeId::new(c, d),
            UndirectedEdgeId::new(d, a),
        ].into_iter().collect());
        let ab = cg.comine_vertices(&[a, b]);
        assert!(ab == a || ab == b);
        assert_eq!(to_edges(&cg), [
            UndirectedEdgeId::new(ab, c),
            UndirectedEdgeId::new(c, d),
            UndirectedEdgeId::new(d, ab),
        ].into_iter().collect());
        let abc = cg.comine_vertices(&[ab, c]);
        assert_eq!(to_edges(&cg), [
            UndirectedEdgeId::new(abc, d),
        ].into_iter().collect());
        cg.comine_vertices(&[abc, d]);
        assert_eq!(to_edges(&cg), HashSet::new());
    }

    #[test]
    fn touching_cycles() {
        let mut graph = UndirectedGraph::new();
        let [a, b, c, d, e, f, g] = graph.add_vertex_array();
        graph.add_edge(a, b);
        graph.add_edge(b, c);
        graph.add_edge(c, a);
        graph.add_edge(c, d);
        graph.add_edge(d, e);
        graph.add_edge(e, f);
        graph.add_edge(f, g);
        graph.add_edge(g, b);
        let mut cg = UndirectedComboGraph::new(graph);
        let bcd = cg.comine_vertices(&[b, c, d]);
        assert_eq!(to_edges(&cg), [
            UndirectedEdgeId::new(a, bcd),
            UndirectedEdgeId::new(bcd, e),
            UndirectedEdgeId::new(e, f),
            UndirectedEdgeId::new(f, g),
            UndirectedEdgeId::new(g, bcd),
        ].into_iter().collect());
    }
}
