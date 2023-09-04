use contest_lib_rs::{io, emitln, undirected_graph::UndirectedGraph, graph::VertexId};

fn dfs(
    g: &UndirectedGraph<(), i32>, v: VertexId, parent_edge_id: i32,
    visited: &mut [bool], len: usize, max_len: &mut usize
) {
    visited[v] = true;
    *max_len = std::cmp::max(*max_len, len);
    for e in g.edges_adj(v) {
        let u = e.other;
        if !visited[u] {
            let edge_id = *e.payload;
            let len_inc = if edge_id > parent_edge_id { 0 } else { 1 };
            dfs(g, u, edge_id, visited, len + len_inc, max_len);
        }
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut g = UndirectedGraph::new();
    g.add_vertices(n);
    for i in 1..n {
        let u = VertexId::from_1_based(read.u32());
        let v = VertexId::from_1_based(read.u32());
        g.add_edge_p(u, v, i as i32);
    }
    let mut visited = vec![false; n];
    let mut max_len = 0;
    dfs(&g, VertexId::from_1_based(1), -1, &mut visited, 1, &mut max_len);
    emitln!(write, max_len);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        6
        4 5
        1 3
        1 2
        3 4
        1 6
        7
        5 6
        2 4
        2 7
        1 3
        1 2
        4 5
        "), "\
        2
        3");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
