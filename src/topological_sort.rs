use crate::directed_graph::DirectedGraph;
use crate::graph::{Graph, VertexId};


struct GraphHasCycles;

#[derive(Clone, Copy, Default, Debug)]
struct VertexState {
    visited: bool,
    active: bool,
}

pub fn topological_sort<VP, EP>(graph: &DirectedGraph<VP, EP>) -> Option<Vec<VertexId>> {
    let mut vertex_state = vec![VertexState::default(); graph.num_vertices()];
    let mut sorted = Vec::new();
    sorted.reserve_exact(graph.num_vertices());
    for v in graph.vertex_ids() {
        if !vertex_state[v].visited {
            match visit(graph, v, &mut vertex_state, &mut sorted) {
                Err(GraphHasCycles) => return None,
                Ok(()) => {}
            }
        }
    }
    sorted.reverse();
    Some(sorted)
}

fn visit<VP, EP>(
    graph: &DirectedGraph<VP, EP>, v: VertexId,
    vertex_state: &mut Vec<VertexState>, sorted: &mut Vec<VertexId>
) -> Result<(), GraphHasCycles> {
    if vertex_state[v].visited {
        return Ok(());
    }
    if vertex_state[v].active {
        return Err(GraphHasCycles {});
    }
    vertex_state[v].active = true;
    for (w, _) in graph.edges_out(v) {
        visit(graph, w, vertex_state, sorted)?;
    }
    vertex_state[v].active = false;
    vertex_state[v].visited = true;
    sorted.push(v);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::directed_graph::*;
    use crate::topological_sort::topological_sort;

    // a → b → c → d
    #[test]
    fn topological_sort_linear() {
        let mut graph = DirectedGraph::new();
        let [a, b, c, d] = graph.add_vertex_array();
        graph.add_edge(a, b);
        graph.add_edge(b, c);
        graph.add_edge(c, d);
        let sorted = topological_sort(&graph).unwrap();
        assert_eq!(sorted.len(), 4);
        assert_eq!(sorted[0], a);
        assert_eq!(sorted[1], b);
        assert_eq!(sorted[2], c);
        assert_eq!(sorted[3], d);
    }

    // a → b → d   →   f
    //   ↘   ↗   ↘   ↗
    //     c       e
    #[test]
    fn topological_sort_dag() {
        let mut graph = DirectedGraph::new();
        let [a, b, c, d, e, f] = graph.add_vertex_array();
        graph.add_edge(a, b);
        graph.add_edge(a, c);
        graph.add_edge(b, d);
        graph.add_edge(c, d);
        graph.add_edge(d, e);
        graph.add_edge(e, f);
        graph.add_edge(d, f);
        let sorted = topological_sort(&graph).unwrap();
        assert_eq!(sorted.len(), 6);
        assert_eq!(sorted[0], a);
        assert_eq!(sorted[3], d);
        assert_eq!(sorted[4], e);
        assert_eq!(sorted[5], f);
    }

    //   →
    // a   b
    //   ←
    #[test]
    fn topological_sort_cycle2() {
        let mut graph = DirectedGraph::new();
        let [a, b] = graph.add_vertex_array();
        graph.add_edge(a, b);
        graph.add_edge(b, a);
        let sorted = topological_sort(&graph);
        assert!(sorted.is_none());
    }

    // a → b  →  c
    //      ↖   ↙
    //        d
    #[test]
    fn topological_sort_cycle3() {
        let mut graph = DirectedGraph::new();
        let [a, b, c, d] = graph.add_vertex_array();
        graph.add_edge(a, b);
        graph.add_edge(b, c);
        graph.add_edge(c, d);
        graph.add_edge(d, b);
        let sorted = topological_sort(&graph);
        assert!(sorted.is_none());
    }
}
