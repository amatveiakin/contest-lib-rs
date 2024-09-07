use contest_lib_rs::base_one::Base;
use contest_lib_rs::graph::{Graph, VertexId};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

fn compute_costs(u: VertexId, t: &Tree<(), ()>, a: &[i64], subtree_sizes: &[i64], costs: &mut Vec<i64>) {
    for v in t.children(u) {
        let cb = a[u] ^ a[v];
        let cuv = cb * subtree_sizes[v];
        let cvu = cb * (t.num_vertices() as i64 - subtree_sizes[v]);
        costs[0] += cuv;
        costs[v] += -cuv + cvu;
        compute_costs(v, t, a, subtree_sizes, costs);
    }
}

fn push_costs(u: VertexId, t: &Tree<(), ()>, costs: &mut Vec<i64>) {
    for v in t.children(u) {
        costs[v] += costs[u];
        push_costs(v, t, costs);
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i64(n);
    let t = Tree::from_read_edges(n, Base::ONE, read);
    let subtree_sizes = t.compute_recursively(|ch_sizes, _| {
        1 + ch_sizes.iter().copied().sum::<i64>()
    });
    let mut costs = vec![0i64; n];
    compute_costs(0, &t, &a, &subtree_sizes, &mut costs);
    push_costs(0, &t, &mut costs);
    emitln!(write, costs);
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
        4
        3 2 1 0
        1 2
        2 3
        2 4
        1
        100
        "), "\
        8 6 12 10
        0 ");
    }
}
