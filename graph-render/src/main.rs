use contest_lib_rs::testing::graph_output::save_graph;
use contest_lib_rs::testing::io_utils::reader_from_string;
use contest_lib_rs::undirected_graph::UndirectedGraph;

fn main() {
    let edges_input = "\
    ";
    let mut read = reader_from_string(edges_input);
    let num_vertices = 0;
    let num_edges = edges_input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).count();
    let t = UndirectedGraph::from_read_edges(num_vertices, num_edges, &mut read);
    save_graph(&t);
}
