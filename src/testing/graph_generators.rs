// Improvement potential: Support max_degree (or max_in_degree + max_out_degree) for directed and
// undirected graphs.

use std::collections::HashSet;

use crate::bfs::bfs_distances;
use crate::directed_graph::DirectedGraph;
use crate::graph::{VertexId, Graph};
use crate::rand::{self, Rng, SliceRandom};
use crate::relax::Relax;
use crate::weakly_connected::weakly_connected_components;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DirectedGraphConnectedness {
    NotConnected,
    WeaklyConnected,
    ReachableFromFirstVertex,
    StronglyConnected,  // not implemented yet
}

pub struct DirectedGraphParams {
    // Minimal connectedness. The graph may be more connected than this, but not less.
    pub ensure_connectedness: DirectedGraphConnectedness,

    // Whether the graph must be a DAG. If false, the graph may or may not be a DAG.
    pub ensure_dag: bool,

    // Whether u->v and v->u edges may coexist (a.k.a. whether length 2 cycles are allowed).
    // Ignored if `ensure_dag` is true.
    pub allow_back_and_fouth_edges: bool,

    // Whether loops (v->v edges) are allowed.
    pub allow_loops: bool,

    // Number of vertices, upper bound inclusive.
    pub min_vertices: u32,
    pub max_vertices: u32,

    // Desired fraction of edges of all possible edges, from 0.0 to 1.0. Actual number may be
    // outside of this range.
    pub min_edge_ratio: f64,
    pub max_edge_ratio: f64,

    // Maximum number of edges, inclusive. Actual number may be slightly higher.
    pub max_edges: Option<u32>,
}

impl Default for DirectedGraphParams {
    fn default() -> Self {
        Self {
            ensure_connectedness: DirectedGraphConnectedness::NotConnected,
            ensure_dag: false,
            allow_back_and_fouth_edges: true,
            allow_loops: false,
            min_vertices: 1,
            max_vertices: 10,
            min_edge_ratio: 0.0,
            max_edge_ratio: 1.0,
            max_edges: None,
        }
    }
}


// TODO:
//
// pub struct UndirectedGraphParams {
//     // Whether the graph must be connected. If false, the graph may or may not be connected.
//     pub ensure_connected: bool,
//
//     // Whether loops (v->v edges) are allowed.
//     pub allow_loops: bool,
//
//     // Number of vertices, upper bound inclusive.
//     pub min_vertices: u32,
//     pub max_vertices: u32,
//
//     // Desired fraction of edges of all possible edges, from 0.0 to 1.0. Actual number may be
//     // outside of this range.
//     pub min_edge_ratio: f64,
//     pub max_edge_ratio: f64,
//
//     // Maximum number of edges, inclusive. Actual number may be slightly higher.
//     pub max_edges: Option<u32>,
// }
//
// impl Default for UndirectedGraphParams {
//     fn default() -> Self {
//         Self {
//             ensure_connected: false,
//             allow_loops: false,
//             min_vertices: 1,
//             max_vertices: 10,
//             min_edge_ratio: 0.0,
//             max_edge_ratio: 1.0,
//             max_edges: None,
//         }
//     }
// }


// TODO (don't forget to check whether `TreeBranchingLimit` note turn out true)
//
// // Vertex degree limit:
// //   - `MaxChildren(N)`: each vertex is allowed to have at move N children;
// //   - `MaxDegree(N)`: each vertex is allowed to have at move N adjacent vertices;
// //     similar to `MaxChildren(N - 1)`, but allows the root to have an extra child;
// //   - `Unlimited`: no limit on vertex degree.
// //
// // Note that `MaxChildren(N)` or `MaxDegree(N)` for very large N would produce different trees
// // compared to `Unlimited`: the former will generate vertices with degrees distributed uniformly
// // from 1 to N; the latter will randomly choose a tree from a uniform distribution over all trees.
// pub enum TreeBranchingLimit {
//     MaxChildren(u32),
//     MaxDegree(u32),
//     Unlimited,
// }
//
// pub struct TreeParams {
//     // Number of vertices, upper bound inclusive.
//     pub min_vertices: u32,
//     pub max_vertices: u32,
//
//     // Vertex degree limit. see `TreeBranchingLimit`.
//     pub branching_limit: TreeBranchingLimit,
//
//     ... something about balance?
//     ... something about depth?
// }
//
// impl Default for TreeParams {
//     fn default() -> Self {
//         Self {
//             min_vertices: 1,
//             max_vertices: 10,
//             branching_limit: TreeBranchingLimit::MaxChildren(2),
//         }
//     }
// }


// TODO: O(E) algorithm for E << V^2. Idea: if V > 100 and E < max(V^2 / 10), then generate random
// edges on the fly instead of precomputing the list of all edges. Note that doing this in all cases
// would be bad, because it's hard to get close a full graph while generating random edges
// independently.
pub fn random_directed_graph(params: DirectedGraphParams) -> DirectedGraph<(), ()> {
    assert!(params.min_vertices <= params.max_vertices);
    assert!(0.0 <= params.min_edge_ratio);
    assert!(params.min_edge_ratio <= params.max_edge_ratio);
    assert!(params.max_edge_ratio <= 1.0);
    if params.ensure_connectedness == DirectedGraphConnectedness::StronglyConnected {
        assert!(!params.ensure_dag);
        assert!(params.allow_back_and_fouth_edges || params.min_vertices >= 3);
    }

    let mut rng = rand::thread_rng();
    let mut g = DirectedGraph::new();

    let num_vertices = rng.int_range_inclusive(params.min_vertices, params.max_vertices) as usize;
    g.add_vertices(num_vertices);
    if num_vertices == 0 {
        return g;
    }

    // If `vertex_order` is `Some`, then all edges in the graph are directed from lower to higher
    // vertex indices.
    let mut all_vertices_in_order = None;
    let mut vertex_order = None;
    let mut all_edges = vec![];
    if params.ensure_dag {
        let mut all_v_in_order = (0..num_vertices).collect::<Vec<_>>();
        let mut v_order = vec![0; num_vertices];
        for i in 0..num_vertices {
            v_order[all_v_in_order[i]] = i;
        }
        if params.ensure_connectedness == DirectedGraphConnectedness::ReachableFromFirstVertex {
            all_v_in_order[1..].shuffle(&mut rng);
        } else {
            all_v_in_order.shuffle(&mut rng);
        }
        for i in 0..num_vertices {
            for j in i..num_vertices {
                let u = all_v_in_order[i];
                let v = all_v_in_order[j];
                if u != v || params.allow_loops {
                    all_edges.push((u, v));
                }
            }
        }
        all_vertices_in_order = Some(all_v_in_order);
        vertex_order = Some(v_order);
    } else {
        for u in 0..num_vertices {
            for v in u..num_vertices {
                if u == v {
                    if params.allow_loops {
                        all_edges.push((u, v));
                    }
                } else {
                    if params.allow_back_and_fouth_edges {
                        all_edges.push((u, v));
                        all_edges.push((v, u));
                    } else {
                        if rng.gen() {
                            all_edges.push((u, v));
                        } else {
                            all_edges.push((v, u));
                        }
                    }
                }
            }
        }
    }
    all_edges.shuffle(&mut rng);
    let mut num_edges = rng.int_range_inclusive(
        (params.min_edge_ratio * all_edges.len() as f64) as u32,
        (params.max_edge_ratio * all_edges.len() as f64) as u32,
    ) as usize;

    num_edges.relax_clamp(0, all_edges.len());
    if let Some(max_edges) = params.max_edges {
        num_edges.relax_min(max_edges as usize);
    }
    for &(u, v) in &all_edges[..num_edges] {
        g.add_edge(u, v);
    }

    match params.ensure_connectedness {
        DirectedGraphConnectedness::NotConnected => {}
        DirectedGraphConnectedness::WeaklyConnected => {
            // Improvement potential: Instead of connecting the components lineary, generate a
            // random tree with `components.len()` vertices and use that.
            let components = weakly_connected_components(&g);
            if components.len() > 1 {
                for i in 0..(components.len() - 1) {
                    let u = *components[i].choose(&mut rng).unwrap();
                    let v = *components[i + 1].choose(&mut rng).unwrap();
                    assert!(g.edge(u, v).is_none());
                    assert!(g.edge(v, u).is_none());
                    if let Some(ref vertex_order) = vertex_order {
                        // Technically, this is supperfluous: we cannot break DAG-ness by adding one
                        // edge between two previously unconnected components. Still, let's uphold
                        // vertex order guarantee in case some processing is added after
                        // connectedness check.
                        if vertex_order[u] < vertex_order[v] {
                            g.add_edge(u, v);
                        } else {
                            g.add_edge(v, u);
                        }
                    } else {
                        if rng.gen() {
                            g.add_edge(u, v);
                        } else {
                            g.add_edge(v, u);
                        }
                    }
                }
            }
        }
        DirectedGraphConnectedness::ReachableFromFirstVertex => {
            let first_v = 0 as VertexId;
            let reachable_set = bfs_distances(&g, first_v)
                .keys().copied().collect::<HashSet<_>>();
            if let Some(ref all_vertices_in_order) = all_vertices_in_order {
                assert_eq!(all_vertices_in_order[0], first_v);
                for i in 1..num_vertices {
                    let v = all_vertices_in_order[i];
                    if !reachable_set.contains(&v) {
                        let u = *all_vertices_in_order[..i].choose(&mut rng).unwrap();
                        g.add_edge(u, v);
                        if !params.allow_back_and_fouth_edges {
                            g.remove_edge(v, u);
                        }
                    }
                }
            } else {
                let mut reachable_vec = reachable_set.iter().copied().collect::<Vec<_>>();
                for v in g.vertex_ids() {
                    if !reachable_set.contains(&v) {
                        let u = *reachable_vec.choose(&mut rng).unwrap();
                        g.add_edge(u, v);
                        if !params.allow_back_and_fouth_edges {
                            g.remove_edge(v, u);
                        }
                        reachable_vec.push(v);
                    }
                }
            }
        }
        DirectedGraphConnectedness::StronglyConnected => {
            assert!(vertex_order.is_none());
            // TODO: Implement strongly connected graph generation.
            unimplemented!();
        }
    }
    g
}


// TODO: Test that generated graphs satisfy the parameters.
