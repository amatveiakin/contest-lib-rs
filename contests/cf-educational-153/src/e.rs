use std::collections::HashMap;

use contest_lib_rs::bfs::bfs_path;
use contest_lib_rs::graph::VertexId;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::undirected_graph::UndirectedGraph;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let s: Vec<_> = read.word().chars().collect();
    let n = s.len();
    let mut pairs = HashMap::new();
    for i in 0..(n - 1) {
        let p = (s[i], s[i+1]);
        if !pairs.contains_key(&p) {
            let new_idx = pairs.len();
            pairs.insert(p, new_idx);
        };
    }
    let mut g = UndirectedGraph::new();
    g.add_vertices(pairs.len());
    for i in 0..(n - 2) {
        let p1 = (s[i],   s[i+1]);
        let p2 = (s[i+1], s[i+2]);
        let idx1 = *pairs.get(&p1).unwrap();
        let idx2 = *pairs.get(&p2).unwrap();
        g.add_edge(VertexId::from_0_based(idx1), VertexId::from_0_based(idx2));

    }
    let q = read.usize();
    for _ in 0..q {
        let [a, b] = read.usizes();
        let p1 = (s[a-1], s[a]);
        let p2 = (s[b-1], s[b]);
        let idx1 = *pairs.get(&p1).unwrap();
        let idx2 = *pairs.get(&p2).unwrap();
        let answer = bfs_path(&g, VertexId::from_0_based(idx1), VertexId::from_0_based(idx2)).unwrap().len();
        // What about cross-cluster moves?
        emitln!(write, answer);
    }
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
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
//         assert_trimmed_eq!(&run_solver(solve, "\
// codecode
// 3
// 1 7
// 3 5
// 3 6
// "), "\
// 3
// 2
// 2");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
