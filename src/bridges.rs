// `find_bridges` identifies edges which, when removed, make the graph disconnected.
//
// Based on https://cp-algorithms.com/graph/bridge-searching.html

use std::collections::HashSet;

use crate::bitset::Bitset;
use crate::graph::{Graph, VertexId};
use crate::undirected_graph::{UndirectedEdgeId, UndirectedGraph};


fn dfs<VP, EP>(
    graph: &UndirectedGraph<VP, EP>, v: VertexId, p: Option<VertexId>,
    visited: &mut Bitset, t_in: &mut Vec<u32>, low: &mut Vec<u32>, t: &mut u32,
    bridges: &mut HashSet<UndirectedEdgeId>,
) {
    visited.set(v, true);
    t_in[v] = *t;
    low[v] = *t;
    *t += 1;
    for (u, _) in graph.edges_adj(v) {
        if Some(u) == p {
            continue;
        }
        if visited.get(u) {
            low[v] = low[v].min(t_in[u]);
        } else {
            dfs(graph, u, Some(v), visited, t_in, low, t, bridges);
            low[v] = low[v].min(low[u]);
            if low[u] > t_in[v] {
                bridges.insert(UndirectedEdgeId::new(v, u));
            }
        }
    }
}

pub fn find_bridges<VP, EP>(graph: &UndirectedGraph<VP, EP>) -> HashSet<UndirectedEdgeId> {
    let n = graph.num_vertices();
    let mut visited = Bitset::new(n);
    let mut t_in = vec![0; n];
    let mut low = vec![0; n];
    let mut t = 0;
    let mut bridges = HashSet::new();
    for v in 0..n {
        if !visited.get(v) {
            dfs(&graph, v, None, &mut visited, &mut t_in, &mut low, &mut t, &mut bridges);
        }
    }
    bridges
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::testing::graph_ext::UndirectedGraphExtension;

    #[test]
    fn just_a_cycle() {
        let mut g = UndirectedGraph::new();
        g.add_cycle::<4>();
        assert_eq!(find_bridges(&g), HashSet::new());
    }

    #[test]
    fn one_bridge() {
        let mut g = UndirectedGraph::new();
        let [_, a, _] = g.add_cycle();
        let [b, _, _] = g.add_cycle();
        g.add_edge(a, b);
        assert_eq!(find_bridges(&g), [UndirectedEdgeId::new(a, b)].into_iter().collect());
    }

    #[test]
    fn tree() {
        let mut g = UndirectedGraph::new();
        let [a, b, c, d, e] = g.add_vertex_array();
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, d);
        g.add_edge(d, e);
        assert_eq!(find_bridges(&g), [
            UndirectedEdgeId::new(a, b),
            UndirectedEdgeId::new(a, c),
            UndirectedEdgeId::new(b, d),
            UndirectedEdgeId::new(d, e),
        ].into_iter().collect());
    }

    #[test]
    fn complex_graph() {
        let mut g = UndirectedGraph::new();
        let [_, a, _, _] = g.add_cycle();
        let [b1, _, b2, _] = g.add_clique();
        let [c, d, e] = g.add_vertex_array();
        g.add_edge(a, c);
        g.add_edge(c, b1);
        g.add_edge(a, d);
        g.add_edge(b2, e);
        assert_eq!(find_bridges(&g), [
            UndirectedEdgeId::new(a, c),
            UndirectedEdgeId::new(c, b1),
            UndirectedEdgeId::new(a, d),
            UndirectedEdgeId::new(b2, e),
        ].into_iter().collect());
        g.add_edge(d, e);
        assert_eq!(find_bridges(&g), HashSet::new());
    }
}
