use std::collections::{BTreeSet, VecDeque};

use contest_lib_rs::base_one::Base;
use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::undirected_graph::UndirectedGraph;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut g = UndirectedGraph::from_read_edges(n, n - 1, Base::ONE, read);

    let mut sz = vec![1; n];
    let mut ch = vec![BTreeSet::new(); n];
    let mut qu = VecDeque::new();

    for v in 0..n {
        if g.degree(v) == 1 {
            qu.push_back(v);
            let [(vp, ())] = g.edges_adj(v).collect_array();
            ch[vp].insert((sz[v], v));
        }
    }

    while let Some(u) = qu.pop_front() {
        if g.degree(u) == 0 {
            continue;
        }
        let [(v, ())] = g.edges_adj(u).collect_array();
        if sz[v] >= sz[u] {
            g.remove_edge(u, v);
            sz[v] += sz[u];
            ch[v].remove(&(sz[u], u));
            if g.degree(v) == 1 {
                qu.push_back(v);
                let [(vp, ())] = g.edges_adj(v).collect_array();
                ch[vp].insert((sz[v], v));
            }
            if let Some(&(_, vch)) = ch[v].first() {
                // assert_eq!(g.degree(vch), 1);
                qu.push_back(vch);
            }
        }
    }

    emitln!(write, (g.num_edges() == 0).YESNO());
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        5 1
        3 2
        4 6
        3 6
        7 1
        1 3"), "YES");
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        1 4
        4 2
        3 2
        5 3"), "NO");
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        4 5
        5 6
        6 1
        2 6
        3 2"), "YES");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
