// TODO: Why is it so slow? It's linear, not even N log N.

use std::collections::HashSet;

use contest_lib_rs::base_one::IteratorBaseOneConversion;
use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::genealogy::VertexDepths;
use contest_lib_rs::graph::VertexId;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::tree::Tree;

fn dfs_answer(
    t: &Tree<(), ()>, v: usize, parent_path: Option<u32>,
    a: &HashSet<VertexId>, depth: &VertexDepths, max_red_depth: &[Option<u32>], min_f: &mut u32
) {
    let f = max_red_depth[v].map_or(0, |d| d - depth[v]).max(parent_path.unwrap_or(0));
    min_f.relax_min(f);
    let mut subtree_red_depths = CountingSet::from_item_iter(t.children(v).filter_map(|ch| max_red_depth[ch]));
    for ch in t.children(v) {
        if let Some(ch_d) = max_red_depth[ch] {
            subtree_red_depths.remove(ch_d);
        }
        let my_path = a.contains(&v).then_some(1);
        let subtree_path = subtree_red_depths.last().map(|d| d - depth[v] + 1);
        let grandparent_path = parent_path.map(|p| p + 1);
        let new_parent_path = [my_path, grandparent_path, subtree_path].into_iter().filter_map(|x| x).max();
        dfs_answer(t, ch, new_parent_path, a, depth, max_red_depth, min_f);
        if let Some(my_d) = max_red_depth[ch] {
            subtree_red_depths.push(my_d);
        }
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let a = read.vec_usize(k).into_iter().from1b().collect::<HashSet<_>>();
    let t = Tree::from_read_edges(n, read).unwrap();

    let depths = VertexDepths::new(&t);
    let max_red_depth = t.compute_recursively(|ch, v| {
        if let Some(ret) = ch.iter().filter_map(|x| **x).max() {
            Some(ret)
        } else if a.contains(&v) {
            Some(depths[v])
        } else {
            None
        }
    });
    let mut min_f = u32::MAX;
    dfs_answer(&t, t.root(), None, &a, &depths, &max_red_depth, &mut min_f);
    emitln!(write, min_f);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        7 3
        2 6 7
        1 2
        1 3
        2 4
        2 5
        3 6
        3 7
        4 4
        1 2 3 4
        1 2
        2 3
        3 4
        5 1
        1
        1 2
        1 3
        1 4
        1 5
        5 2
        4 5
        1 2
        2 3
        1 4
        4 5
        10 8
        1 2 3 4 5 8 9 10
        2 10
        10 5
        5 3
        3 1
        1 7
        7 4
        4 9
        8 9
        6 1
        10 9
        1 2 4 5 6 7 8 9 10
        1 3
        3 9
        9 4
        4 10
        10 6
        6 7
        7 2
        2 5
        5 8
        "), "\
        2
        2
        0
        1
        4
        5");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        6 1
        3
        1 2
        1 3
        3 4
        3 5
        2 6
        5 3
        1 2 5
        1 2
        1 3
        2 4
        3 5
        7 1
        2
        3 2
        2 6
        6 1
        5 6
        7 6
        4 5
        "), "\
        0
        2
        0");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
