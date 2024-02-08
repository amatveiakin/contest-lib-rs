// `strongly_connected_components` returns strongly connected components of a directed graph.
//
// Based on https://cp-algorithms.com/graph/strongly-connected-components.html

use crate::bitset::Bitset;
use crate::directed_graph::DirectedGraph;
use crate::graph::{Graph, VertexId};


fn dfs1<VP, EP>(g: &DirectedGraph<VP, EP>, v: VertexId, visited: &mut Bitset, order: &mut Vec<VertexId>) {
    visited.set(v, true);
    for (u, _) in g.edges_out(v) {
        if !visited.get(u) {
            dfs1(g, u, visited, order);
        }
    }
    order.push(v);
}

fn dfs2<VP, EP>(g: &DirectedGraph<VP, EP>, v: VertexId, visited: &mut Bitset, component: &mut Vec<VertexId>) {
    visited.set(v, true);
    component.push(v);
    for (u, _) in g.edges_in(v) {
        if !visited.get(u) {
            dfs2(g, u, visited, component);
        }
    }
}

pub fn strongly_connected_components<VP, EP>(g: &DirectedGraph<VP, EP>) -> Vec<Vec<VertexId>> {
    let mut visited = Bitset::new(g.num_vertices());
    let mut order = vec![];
    for v in g.vertex_ids() {
        if !visited.get(v) {
            dfs1(g, v, &mut visited, &mut order);
        }
    }

    visited.fill(false);
    let mut components = Vec::new();
    for &v in order.iter().rev() {
        let mut component = Vec::new();
        if !visited.get(v) {
            dfs2(g, v, &mut visited, &mut component);
            components.push(component);
        }
    }
    components
}


#[cfg(test)]
mod tests {
    use crate::testing::graph_ext::DirectedGraphExtension;

    use super::*;

    fn components_sorted<VP, EP>(g: &DirectedGraph<VP, EP>) -> Vec<Vec<VertexId>> {
        let mut components = strongly_connected_components(g);
        for component in &mut components {
            component.sort_unstable();
        }
        components.sort_unstable();
        components
    }

    #[test]
    fn closing_cycle() {
        let mut graph = DirectedGraph::new();
        let [a, b, c, d] = graph.add_unidirectional_chain();
        assert_eq!(components_sorted(&graph), vec![vec![a], vec![b], vec![c], vec![d]]);
        graph.add_edge(d, a);
        assert_eq!(components_sorted(&graph), vec![vec![a, b, c, d]]);
    }

    #[test]
    fn dag_plus_cycle() {
        let mut graph = DirectedGraph::new();
        let [a, b, c, d] = graph.add_dag_clique();
        assert_eq!(components_sorted(&graph), vec![vec![a], vec![b], vec![c], vec![d]]);
        graph.add_edge(d, a);
        assert_eq!(components_sorted(&graph), vec![vec![a, b, c, d]]);
    }

    #[test]
    fn two_components() {
        let mut graph = DirectedGraph::new();
        let [a, b, c] = graph.add_full_clique();
        let [d, e, f] = graph.add_bidirectional_cycle();
        assert_eq!(components_sorted(&graph), vec![vec![a, b, c], vec![d, e, f]]);
        graph.add_edge(a, d);
        assert_eq!(components_sorted(&graph), vec![vec![a, b, c], vec![d, e, f]]);
        graph.add_edge(e, b);
        assert_eq!(components_sorted(&graph), vec![vec![a, b, c, d, e, f]]);
    }
}
