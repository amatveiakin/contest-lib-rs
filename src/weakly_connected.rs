// Provides `weakly_connected_components` which returns:
//   - weakly connected components of a directed graph;
//   - connected components of an undirected graph.
//
// Note. From an API perspective it would've been nice to provide a separate `connected_components`
// function for undirected graphs, but this would required pulling in the `undirected_graph` module,
// and at this point the collator extension is not smart enough to omit it when not used.

use crate::bitset::Bitset;
use crate::graph::{Graph, VertexId};


pub fn weakly_connected_components<VP, EP>(g: &impl Graph<VP, EP>) -> Vec<Vec<VertexId>> {
    let mut components = Vec::new();
    let mut visited = Bitset::new(g.num_vertices());
    let mut stack = Vec::new();
    for v in g.vertex_ids() {
        if visited.get(v) {
            continue;
        }
        stack.push(v);
        let mut component = Vec::new();
        while let Some(v) = stack.pop() {
            if !visited.get(v) {
                visited.set(v, true);
                component.push(v);
                for (w, _) in g.edges_adj(v) {
                    stack.push(w);
                }
            }
        }
        components.push(component);
    }
    components
}


#[cfg(test)]
mod tests {
    use crate::directed_graph::DirectedGraph;

    use super::*;

    fn components_sorted(mut components: Vec<Vec<VertexId>>) -> Vec<Vec<VertexId>> {
        for component in &mut components {
            component.sort();
        }
        components.sort();
        components
    }

    #[test]
    fn connected() {
        let mut g = DirectedGraph::new();
        let [a, b, c, d, e] = g.add_vertex_array();
        g.add_edge(a, b);
        g.add_edge(b, c);
        g.add_edge(a, d);
        g.add_edge(b, e);
        g.add_edge(d, e);
        assert_eq!(components_sorted(weakly_connected_components(&g)), vec![
            vec![a, b, c, d, e]
        ]);
    }

    #[test]
    fn not_connected() {
        let mut g = DirectedGraph::new();
        let [a, b, c, d, e] = g.add_vertex_array();
        g.add_edge(a, c);
        g.add_edge(b, d);
        g.add_edge(d, e);
        g.add_edge(e, b);
        assert_eq!(components_sorted(weakly_connected_components(&g)), vec![
            vec![a, c],
            vec![b, d, e]
        ]);
    }
}
