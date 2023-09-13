use crate::graph::Graph;
use crate::testing::graphviz::to_graphviz;

use super::type_utils::is_zst;


// Graphs without payload can be read back via `from_read_edges`.
pub fn graph_to_string<VP: std::fmt::Debug, EP: std::fmt::Debug>(g: &impl Graph<VP, EP>) -> String {
    let mut s = String::new();
    s.push_str(&format!("num_vertices = {}\n", g.num_vertices()));
    if !is_zst::<VP>() {
        for v in g.vertex_ids() {
            s.push_str(&format!("{} {:?}\n", v.to_1_based(), g.vertex(v)));
        }
    }
    s.push_str(&format!("num_edges = {}\n", g.num_edges()));
    if is_zst::<EP>() {
        for (u, v, _) in g.edges() {
            s.push_str(&format!("{} {}\n", u.to_1_based(), v.to_1_based()));
        }
    } else {
        for (u, v, payload) in g.edges() {
            s.push_str(&format!("{} {} {:?}\n", u.to_1_based(), v.to_1_based(), payload));
        }
    }
    s
}

pub fn save_graph<VP: std::fmt::Debug, EP: std::fmt::Debug>(g: &impl Graph<VP, EP>) {
    std::fs::write("graph.txt", graph_to_string(g)).unwrap();
    std::fs::write("graph.gv", to_graphviz(g)).unwrap();
}
