// UNFINISHED

use contest_lib_rs::graph::VertexId;
use contest_lib_rs::io;
use contest_lib_rs::tree::Tree;
use contest_lib_rs::undirected_graph::UndirectedGraph;

fn compute_subtree_sizes(v: VertexId, tree: &Tree<(), ()>, subtree_sizes: &mut Vec<usize>) -> usize {
    let mut size = 1;
    for &u in tree.children(v) {
        size += compute_subtree_sizes(u, tree, subtree_sizes);
    }
    subtree_sizes[v] = size;
    size
}

fn compute_transitive_children(v: VertexId, tree: &Tree<(), ()>, transitive_children: &mut Vec<Vec<VertexId>>) {
    let mut children = Vec::new();
    for &u in tree.children(v) {
        compute_transitive_children(u, tree, transitive_children);
        children.push(u);
        children.extend(transitive_children[u].iter().copied());
    }
    assert!(transitive_children[v].is_empty());
    transitive_children[v] = children;
}

fn dfs(v: VertexId, tree: &Tree<(), ()>, subtree_sizes: &mut Vec<usize>, black: &mut Vec<bool>, k: usize) {
    // also include `v`
    // also invert (?)

    if subtree_sizes[v] == k {
        black.fill(false);

    }
    for &u in tree.children(v) {
        dfs(u, tree, subtree_sizes, black, k);
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut graph = UndirectedGraph::new();
    graph.add_vertices(n);
    for _ in 0..n {
        let from = read.u32();
        let to = read.u32();
        graph.add_edge(VertexId::from_1_based(from), VertexId::from_1_based(to));
    }
    let tree = Tree::from(&graph).unwrap();
    let mut subtree_sizes = vec![0; n];
    let mut transitive_children = vec![vec![]; n];
    compute_subtree_sizes(tree.root(), &tree, &mut subtree_sizes);
    compute_transitive_children(tree.root(), &tree, &mut transitive_children);
    let mut black = vec![false; n];
    for k in 0..=n {
        dfs(tree.root(), &tree, &mut subtree_sizes, &mut black, k)
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
