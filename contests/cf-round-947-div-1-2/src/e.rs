use std::collections::HashSet;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::graph::{Graph, VertexId};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

fn update(
    u: VertexId,
    t: &Tree<(), ()>,
    cl: &mut Vec<i32>,
    nch: &mut Vec<i32>,
    nv: &mut i32,
    ne: &mut i32,
    v2: &mut HashSet<VertexId>,
    nv3: &mut i32,
) {
    cl[u] = 1 - cl[u];
    let sign = if cl[u] == 1 { 1 } else { -1 };
    *nv += sign;
    *ne += sign * nch[u];
    if let Some(v) = t.parent(u) {
        if cl[v] == 1 {
            *ne += sign;
        }
        let onch = nch[v];
        nch[v] += sign;
        if nch[v] == 2 {
            assert!(v2.insert(v))
        } else if onch == 2 {
            assert!(v2.remove(&v));
        }
        if nch[v] >= 3 && onch < 3 {
            *nv3 += 1;
        } else if nch[v] < 3 && onch >= 3 {
            *nv3 -= 1;
        }
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, q] = read.usizes();
    let clinit = read.vec_u32(n);
    let t = Tree::from_read_edges(n, read).unwrap();

    let mut cl = vec![0; n];
    let mut nch = vec![0; n];
    let mut nv = 0;
    let mut ne = 0;
    let mut v2 = HashSet::new();
    let mut nv3 = 0;

    for u in t.vertex_ids() {
        if clinit[u] == 1 {
            update(u, &t, &mut cl, &mut nch, &mut nv, &mut ne, &mut v2, &mut nv3);
        }
    }

    for _ in 0..q {
        let u = read.usize().from1b();
        update(u, &t, &mut cl, &mut nch, &mut nv, &mut ne, &mut v2, &mut nv3);
        let ok =
            nv > 0 &&
            ne == nv - 1 &&
            nv3 == 0 &&
            (
                v2.len() == 0 ||
                (v2.len() == 1 && t.parent(*v2.iter().next().unwrap()).map_or(true, |p| cl[p] == 0))
            )
        ;
        emitln!(write, ok.YesNo());
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
        2 1
        1 0
        1 2
        1
        5 4
        1 0 0 0 0
        1 2
        1 3
        1 5
        3 4
        4
        3
        2
        5
        "), "\
        No
        No
        Yes
        Yes
        No");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        5 3
        1 1 1 1 1
        3 5
        2 5
        3 4
        1 5
        1
        1
        1
        4 4
        0 0 0 0
        1 2
        2 3
        1 4
        1
        2
        3
        2
        1 1
        1
        1
        1 1
        0
        1
        "), "\
        Yes
        No
        Yes
        Yes
        Yes
        Yes
        No
        No
        Yes");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
