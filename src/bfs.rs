use std::ops;
use std::collections::{hash_map, HashSet, HashMap, VecDeque};

use crate::graph::{Graph, VertexId};


// Shortest path from `from` to `to` in an unweighted graph.
pub fn bfs_path<'g, VP, EP: 'g>(
    graph: &'g impl Graph<'g, VP, EP>, from: VertexId, to: VertexId,
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

// Distances from `from` to reachable vertices in an unweighted graph.
pub fn bfs_distances<'g, VP, EP: 'g>(
    graph: &'g impl Graph<'g, VP, EP>, from: VertexId,
) -> HashMap<VertexId, u32> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    distances.insert(from, 0);
    queue.push_back(from);
    while let Some(v) = queue.pop_front() {
        let d = distances[&v];
        for e in graph.edges_from(v) {
            match distances.entry(e.other) {
                hash_map::Entry::Occupied(_) => {}
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(d + 1);
                    queue.push_back(e.other);
                }
            }
        }
    }
    distances
}


#[cfg(test)]
mod tests {
    use crate::directed_graph::*;
    use crate::undirected_graph::*;
    use super::*;

    #[test]
    fn bfs_directed_graph() {
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

        let distances = bfs_distances(&g, a);
        assert_eq!(distances[&a], 0);
        assert_eq!(distances[&b], 1);
        assert_eq!(distances[&c], 1);
        assert_eq!(distances[&d], 2);
        assert_eq!(distances[&e], 3);
    }

    #[test]
    fn bfs_undirected_graph() {
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

        let distances = bfs_distances(&g, a);
        assert_eq!(distances[&a], 0);
        assert_eq!(distances[&b], 1);
        assert_eq!(distances[&c], 1);
        assert_eq!(distances[&d], 2);
        assert_eq!(distances[&e], 2);
    }
}
