use std::cmp;

use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::graph::{Graph, VertexId};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::mex::get_mex;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let m = read.u64();
    let lss = (0..n).map(|_| {
        let len = read.usize();
        read.vec_u64(len)
    }).collect_vec();

    let mut g = DirectedGraph::new();
    for ls in lss.iter() {
        let a = get_mex(ls.iter().copied());
        let mut ls = ls.clone();
        ls.push(a);
        let b = get_mex(ls.iter().copied());
        assert!(b > a);
        let a = a as VertexId;
        let b = b as VertexId;
        g.fit_vertex(a);
        g.fit_vertex(b);
        if let Some(p) = g.edge_mut(a, b) {
            *p += 1;
        } else {
            g.add_edge_p(a, b, 1u32);
        }
    }

    let s = g.num_vertices() - 1;
    let mut r = (0..=s).collect_vec();
    for u in (0..=s).rev() {
        for (v, _) in g.edges_out(u) {
            let rv = r[v];
            r[u].relax_max(rv);
        }
    }

    let mut bdef = 0;
    for i in 0..=s {
        match g.edges_out(i).map(|(_, c)| c).sum() {
            0 => {}
            1 => {
                bdef.relax_max(i);
            }
            2.. => {
                bdef.relax_max(r[i]);
            }
        }
    }

    let s = s as u64;
    let mut ans: u64 = 0;
    for i in 0..=cmp::min(m, s) {
        ans += cmp::max(bdef, r[i as usize]) as u64;
    }
    if m > s {
        ans += m * (m+1) / 2 - s * (s+1) / 2;
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
        // assert_trimmed_eq!(&run_solver(solve_case, "2 50  2 1 2  2 1 2"), "1281");
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        3 4
        2 0 2
        3 2 3 3
        4 7 0 1 5
        3 4
        5 0 2 0 4 11
        1 1
        5 1 3 0 3 3
        2 50
        2 1 2
        2 1 2
        1 1
        7 1 2 4 1 4 9 5
        4 114514
        2 2 2
        5 7 3 6 0 3
        3 0 1 1
        5 0 9 2 1 5
        5 1919810
        1 2
        2 324003 0
        3 1416324 2 1460728
        4 1312631 2 0 1415195
        5 1223554 192248 2 1492515 725556"), "\
        16
        18
        1281
        4
        6556785365
        1842836177961");
    }
}
