use std::cmp;
use std::collections::HashSet;

use contest_lib_rs::base_one::Base;
use contest_lib_rs::graph::{Graph, VertexId};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::joint_sort::joint_sort_by_key;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::undirected_graph::UndirectedGraph;

fn dfs(graph: &UndirectedGraph<(), i64>, k: usize, visited: &mut HashSet<VertexId>, v: VertexId) -> (i64, i64) {
    visited.insert(v);
    let mut weights1 = Vec::new();
    let mut weights2 = Vec::new();
    for (u, &weight) in graph.edges_adj(v) {
        if !visited.contains(&u) {
            let (w1, w2) = dfs(graph, k, visited, u);
            weights1.push(w1 + weight);
            weights2.push(w2 + weight);
        }
    }
    visited.remove(&v);
    let n = weights1.len();
    if n == 0 {
        (0, 0)
    } else {
        joint_sort_by_key(&mut weights1, |&x| cmp::Reverse(x), &mut weights2);
        let r1 = weights1.iter().take(k - 1).sum::<i64>();
        let mut r2 = 0;
        for i in 0..n {
            if i < k - 1 {
                r2.relax_max(r1 - weights1[i] + weights2[i] + weights1.get(k - 1).unwrap_or(&0));
            } else {
                r2.relax_max(r1 + weights2[i]);
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
