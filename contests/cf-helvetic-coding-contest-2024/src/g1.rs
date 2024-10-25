use contest_lib_rs::base_one::Base;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::tree::Tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let c = read.u64();
    let tree = Tree::from_read_edges(n, Base::ONE, read);
    let subtree_sizes = tree.compute_recursively(|ch_sizes, _| {
        1 + ch_sizes.iter().copied().sum::<u64>()
    });
    let mut min_fund = u64::MAX;
    for v in tree.vertex_ids() {
        let x = subtree_sizes[v];
        let y = n as u64 - x;
        min_fund.relax_min(x * x + y * y);
    }
    emitln!(write, min_fund);
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
    }
}
