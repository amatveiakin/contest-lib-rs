use std::cmp;
use std::collections::{hash_map, HashMap, BinaryHeap};

use crate::graph::{Graph, VertexId};


pub struct DijkstraPath {
    pub cost: u64,
    pub path: Vec<VertexId>,  // including `from` and `to`
}

struct VisitedVertex {
    prev: VertexId,
    cost: u64,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct QueuedVertex {
    vertex: VertexId,
    cost: u64,
}

impl Ord for QueuedVertex {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Flip cost ordering for min-heap
        other.cost.cmp(&self.cost).then_with(|| self.vertex.cmp(&other.vertex))
    }
}
impl PartialOrd for QueuedVertex {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> { Some(self.cmp(other)) }
}

// Shortest path from `from` to `to` in an weighted graph.
// Complexity: O(|E| + |V| log |V|).
pub fn dijkstra_path<VP, EP>(
    graph: &impl Graph<VP, EP>, from: VertexId, to: VertexId,
    edge_cost: impl Fn(&EP) -> u64,
) -> Option<DijkstraPath> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    visited.insert(from, VisitedVertex { prev: from, cost: 0 });
    queue.push(QueuedVertex { vertex: from, cost: 0 });
    while let Some(QueuedVertex { vertex: v, cost }) = queue.pop() {
        if v == to {
            let mut path = Vec::new();
            let mut v = v;
            while v != from {
                path.push(v);
                v = visited[&v].prev;
            }
            path.push(from);
            path.reverse();
            return Some(DijkstraPath { cost, path });
        }
        if visited.get(&v).map_or(false, |prior_visit| prior_visit.cost < cost) {
            continue;
        }
        for (w, payload) in graph.edges_out(v) {
            let new_cost = cost + edge_cost(payload);
            match visited.entry(w) {
                hash_map::Entry::Occupied(mut entry) => {
                    if entry.get().cost > new_cost {
                        entry.insert(VisitedVertex { prev: v, cost: new_cost });
                        queue.push(QueuedVertex { vertex: w, cost: new_cost });
                    }
                }
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(VisitedVertex { prev: v, cost: new_cost });
                    queue.push(QueuedVertex { vertex: w, cost: new_cost });
                }
            }
        }
    }
    None
}

// Distances from `from` to reachable vertices in an weighted graph.
// Complexity: O(|E| + |V| log |V|).
pub fn dijkstra_distances<VP, EP>(
    graph: &impl Graph<VP, EP>, from: VertexId, edge_cost: impl Fn(&EP) -> u64,
) -> HashMap<VertexId, u64> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    visited.insert(from, VisitedVertex { prev: from, cost: 0 });
    queue.push(QueuedVertex { vertex: from, cost: 0 });
    while let Some(QueuedVertex { vertex: v, cost }) = queue.pop() {
        if visited.get(&v).map_or(false, |prior_visit| prior_visit.cost < cost) {
            continue;
        }
        for (w, payload) in graph.edges_out(v) {
            let new_cost = cost + edge_cost(payload);
            match visited.entry(w) {
                hash_map::Entry::Occupied(mut entry) => {
                    if entry.get().cost > new_cost {
                        entry.insert(VisitedVertex { prev: v, cost: new_cost });
                        queue.push(QueuedVertex { vertex: w, cost: new_cost });
                    }
                }
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(VisitedVertex { prev: v, cost: new_cost });
                    queue.push(QueuedVertex { vertex: w, cost: new_cost });
                }
            }
        }
    }
    visited.into_iter().map(|(id, visited_vertex)| (id, visited_vertex.cost)).collect()
}


#[cfg(test)]
mod tests {
    use crate::directed_graph::*;
    use super::*;

    #[test]
    fn dijkstra_simple() {
        let mut graph = DirectedGraph::new();
        let [a, b, c] = graph.add_vertex_array();
        graph.add_edge_p(a, b, 1);
        graph.add_edge_p(b, c, 1);
        graph.add_edge_p(a, c, 10);

        let path = dijkstra_path(&graph, a, c, |&w| w).unwrap();
        assert_eq!(path.cost, 2);
        assert_eq!(path.path, vec![a, b, c]);

        let distances = dijkstra_distances(&graph, a, |&w| w);
        assert_eq!(distances[&a], 0);
        assert_eq!(distances[&b], 1);
        assert_eq!(distances[&c], 2);
    }

    #[test]
    fn dijkstra_disconnected() {
        let mut graph = DirectedGraph::new();
        let [a, b, c] = graph.add_vertex_array();
        graph.add_edge_p(a, b, 1);
        graph.add_edge_p(c, b, 1);

        let path = dijkstra_path(&graph, a, c, |&w| w);
        assert!(path.is_none());

        let distances = dijkstra_distances(&graph, a, |&w| w);
        assert_eq!(distances.get(&a), Some(&0));
        assert_eq!(distances.get(&b), Some(&1));
        assert_eq!(distances.get(&c), None);
    }

    // TODO: More tests
}
