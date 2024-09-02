use std::collections::HashMap;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::genealogy::{tree_path_via_depths, VertexDepths};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_windows::IterutilsWindows;
use contest_lib_rs::tree::Tree;

#[derive(Clone, Debug, Default)]
struct VertexStats {
    fixed_sum: u64,
    free_edges: usize,
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let w = read.u64();
    let (mut t, _) = Tree::new_with_root();
    for _ in 0..(n - 1) {
        let p = read.usize().from1b();
        t.add_child(p);
    }

    let vd = VertexDepths::new(&t);
    let mut vst = vec![VertexStats::default(); n];
    let mut e2v = HashMap::new();
    for u in 0..n {
        let v = (u + 1) % n;
        let path = tree_path_via_depths(&t, &vd, u, v);
        for [x1, x2] in path.iter().copied().array_windows() {
            e2v.insert((x1, x2), u);
        }
        vst[u] = VertexStats {
            fixed_sum: 0,
            free_edges: path.len() - 1,
        };
    }

    let mut total = w * (n as u64);
    let mut free_weight = w;
    let mut free_paths = n;
    let mut ans = vec![];
    for _ in 0..(n - 1) {
        let x1 = read.usize().from1b();
        let y = read.u64();
        let x2 = t.parent(x1).unwrap();

        let mut unaffected_free_paths = 0;
        for e in [(x1, x2), (x2, x1)] {
            let v: usize = e2v[&e];
            let st = &mut vst[v];

            st.free_edges -= 1;
            st.fixed_sum += y;
            if st.free_edges == 0 {
                free_paths -= 1;
                total += y;
                total -= free_weight;
            } else {
                unaffected_free_paths += 1;
            }
        }
        free_weight -= y;
        total -= ((free_paths - unaffected_free_paths) as u64) * y;
        ans.push(total);
    }
    emitln!(write, ans);
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
        4
        2 1000000000000
        1
        2 1000000000000
        4 9
        1 1 1
        2 2
        4 4
        3 3
        6 100
        1 2 3 2 1
        6 17
        3 32
        2 4
        4 26
        5 21
        10 511
        1 2 2 4 2 1 1 8 8
        3 2
        6 16
        10 256
        9 128
        2 1
        5 8
        8 64
        4 4
        7 32"), "\
        2000000000000
        25 18 18
        449 302 247 200 200
        4585 4473 2681 1567 1454 1322 1094 1022 1022");
    }
}
