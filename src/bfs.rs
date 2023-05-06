use std::collections::{hash_map, HashSet, HashMap, VecDeque};

use crate::graph::{Graph, VertexId};


// Shortest path from `from` to `to` in an unweighted graph.
pub fn bfs_path<'g, VP, EP: 'g>(
    graph: &'g impl Graph<'g, VP, EP>, from: VertexId, to: VertexId
) -> Option<Vec<VertexId>> {
    let mut queue = VecDeque::new();
    let mut prev = HashMap::new();
    prev.insert(from, from); // `prev` is also used as a visited set
    queue.push_back(from);
    while let Some(v) = queue.pop_front() {
        if v == to {
            let mut path = Vec::new();
            let mut v = v;
            while v != from {
                path.push(v);
                v = prev[&v];
            }
            path.push(from);
            path.reverse();
            return Some(path);
        }
        for e in graph.edges_from(v) {
            match prev.entry(e.other) {
                hash_map::Entry::Occupied(_) => {}
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(v);
                    queue.push_back(e.other);
                }
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use crate::directed_graph::*;
    use crate::undirected_graph::*;
    use super::*;

    #[test]
    fn bfs_path_directed() {
        let mut g = DirectedGraph::new();
        let [a, b, c, d, e] = g.add_vertex_array();
        g.add_edge(a, b);
        g.add_edge(b, c);
        g.add_edge(a, c);
        g.add_edge(c, d);
        g.add_edge(d, e);
        g.add_edge(e, c);
        assert_eq!(bfs_path(&g, a, e), Some(vec![a, c, d, e]));
        assert_eq!(bfs_path(&g, e, a), None);
        assert_eq!(bfs_path(&g, c, c), Some(vec![c]));
        assert_eq!(bfs_path(&g, c, d), Some(vec![c, d]));
    }

    #[test]
    fn bfs_path_undirected() {
        let mut g = UndirectedGraph::new();
        let [a, b, c, d, e] = g.add_vertex_array();
        g.add_edge(a, b);
        g.add_edge(b, c);
        g.add_edge(a, c);
        g.add_edge(c, d);
        g.add_edge(d, e);
        g.add_edge(e, c);
        assert_eq!(bfs_path(&g, a, e), Some(vec![a, c, e]));
        assert_eq!(bfs_path(&g, e, a), Some(vec![e, c, a]));
        assert_eq!(bfs_path(&g, c, c), Some(vec![c]));
        assert_eq!(bfs_path(&g, c, d), Some(vec![c, d]));
    }
}
