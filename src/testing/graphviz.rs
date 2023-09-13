use std::fmt;

use crate::graph::Graph;

use super::type_utils::is_zst;


pub fn to_graphviz<VP: fmt::Debug, EP: fmt::Debug>(g: &impl Graph<VP, EP>) -> String {
    let mut s = String::new();
    let graphviz_type = if g.is_directed() { "digraph" } else { "graph" };
    s.push_str(&format!("{} {{\n", graphviz_type));
    for v in g.vertex_ids() {
        let label = if is_zst::<VP>() {
            "".to_string()
        } else {
            format!(" [label=\"{}\"]", to_debug_escaped(g.vertex(v)))
        };
        s.push_str(&format!("    v{}{};\n", v.to_1_based(), label));
    }
    for (u, v, payload) in g.edges() {
        let label = if is_zst::<EP>() {
            "".to_string()
        } else {
            format!(" [label=\"{}\"]", to_debug_escaped(payload))
        };
        let connector = if g.is_directed() { "->" } else { "--" };
        s.push_str(&format!("    v{} {} v{}{};\n", u.to_1_based(), connector, v.to_1_based(), label));
    }
    s.push_str("}\n");
    s
}

fn to_debug_escaped<T: fmt::Debug>(t: &T) -> String {
    let s = format!("{:?}", t);
    s.escape_default().to_string()
}
