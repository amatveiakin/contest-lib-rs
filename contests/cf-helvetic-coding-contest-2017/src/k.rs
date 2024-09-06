use std::cmp;
use std::collections::HashSet;

use contest_lib_rs::base_one::Base;
use contest_lib_rs::graph::{Graph, VertexId};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::undirected_graph::UndirectedGraph;

struct ChildWeights {
    w1: i64,
    w2: i64,
}

fn dfs(graph: &UndirectedGraph<(), i64>, k: usize, visited: &mut HashSet<VertexId>, v: VertexId) -> (i64, i64) {
    visited.insert(v);
    let mut weights = Vec::new();
    for (u, &weight) in graph.edges_adj(v) {
        if !visited.contains(&u) {
            let (w1, w2) = dfs(graph, k, visited, u);
            weights.push(ChildWeights{
                w1: w1 + weight,
                w2: w2 + weight,
            });
        }
    }
    visited.remove(&v);
    if weights.is_empty() {
        (0, 0)
    } else {
        weights.sort_unstable_by_key(|x| cmp::Reverse(x.w1));
        let r1 = weights.iter().take(k - 1).map(|x| x.w1).sum::<i64>();
        let mut r2 = 0;
        for i in 0..weights.len() {
            if i < k - 1 {
                r2.relax_max(r1 - weights[i].w1 + weights[i].w2 + weights.get(k - 1).map_or(0, |x| x.w1));
            } else {
                r2.relax_max(r1 + weights[i].w2);
            }
        }
        (r1, r2)
    }
}


#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let graph = UndirectedGraph::from_read_edges_p(n, n - 1, Base::ZERO, read, |r| r.i64());
    let mut visited = HashSet::new();
    let (r1, r2) = dfs(&graph, k, &mut visited, 0);
    emitln!(write, cmp::max(r1, r2));
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        9 3
        0 1 1
        0 2 1
        1 3 2
        1 4 2
        1 5 2
        2 6 3
        2 7 3
        2 8 3"), "15");
        assert_trimmed_eq!(&run_solver(solve, "\
        9 5
        0 1 1
        0 2 1
        1 3 2
        1 4 2
        1 5 2
        2 6 3
        2 7 3
        2 8 3"), "17");
        assert_trimmed_eq!(&run_solver(solve, "\
        11 6
        1 0 7932
        2 1 1952
        3 2 2227
        4 0 9112
        5 4 6067
        6 0 6786
        7 6 3883
        8 4 7137
        9 1 2796
        10 5 6200
        "), "54092");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
