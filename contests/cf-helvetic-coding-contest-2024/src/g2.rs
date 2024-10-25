use std::cmp;

use contest_lib_rs::base_one::Base;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::undirected_graph::UndirectedGraph;
use contest_lib_rs::weakly_connected::weakly_connected_components;

fn max_choice(v: &[usize], budget: usize) -> usize {
    let mut f = vec![0; budget + 1];
    for i in 0..v.len() {
        for j in (v[i]..=budget).rev() {
            f[j] = cmp::max(f[j], f[j - v[i]] + v[i]);
        }
    }
    f[budget]
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let c = read.u64();
    let graph = UndirectedGraph::from_read_edges(n, m, Base::ONE, read);
    let new_edge_cost = (weakly_connected_components(&graph).len() - 1) as u64 * c;
    let mut comp_cost = u64::MAX;
    for (u, v, _) in graph.edges() {
        let mut graph2 = graph.clone();
        graph2.remove_edge(u, v);
        let comp_sizes = weakly_connected_components(&graph2).iter().map(|c| c.len()).collect_vec();
        if comp_sizes.len() > 1 {
            let x = max_choice(&comp_sizes, ((n + 1) / 2) as usize);
            let y = n - x as usize;
            comp_cost.relax_min((x * x + y * y) as u64);
        }
    }
    if comp_cost == u64::MAX {
        emitln!(write, -1);
    } else {
        emitln!(write, comp_cost + new_edge_cost);
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
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
        2
        2 1 3
        1 2
        8 7 76
        3 1
        3 2
        2 4
        2 5
        4 6
        4 7
        7 8"), "\
        2
        32");

        assert_trimmed_eq!(&run_solver(solve, "\
        4
        4 6 5
        4 3
        2 3
        2 4
        1 2
        4 1
        3 1
        6 6 2
        1 4
        2 5
        3 6
        1 5
        3 5
        6 5
        6 5 7
        1 4
        2 5
        3 6
        3 5
        6 5
        7 5 4
        1 4
        3 6
        3 5
        6 5
        2 7"), "\
        -1
        20
        25
        33");

        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
