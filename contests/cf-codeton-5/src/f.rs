// UNFINISHED

use contest_lib_rs::graph::VertexId;
use contest_lib_rs::io;
use contest_lib_rs::tree::Tree;

fn dfs(v: VertexId, tree: &Tree<(), ()>, subtree_sizes: &[i64], black: &mut Vec<bool>, k: usize) {
    // also include `v`
    // also invert (?)

    if subtree_sizes[v] == k as i64 {
        black.fill(false);

    }
    for u in tree.children(v) {
        dfs(u, tree, subtree_sizes, black, k);
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let tree = Tree::from_read_edges(n, read).unwrap();
    let subtree_sizes = tree.compute_recursively(|ch_sizes, _| {
        1 + ch_sizes.iter().copied().sum::<i64>()
    });
    let subtrees = tree.compute_recursively(|ch_subtrees, v| {
        let mut ret = vec![v];
        for ch in ch_subtrees {
            ret.extend(*ch);
        }
        ret.sort();
        ret
    });
    let mut black = vec![false; n];
    for k in 0..=n {
        dfs(tree.root(), &tree, &subtree_sizes, &mut black, k)
    }
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
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
