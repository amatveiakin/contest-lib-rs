use contest_lib_rs::base_one::Base;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::num::RingNumber;
use contest_lib_rs::sparse_matrix::SparseMatrix;
use contest_lib_rs::tree::Tree;

type ModNum = ModNumber<1_000_000_007>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    const EXCLUDE: usize = usize::MAX;

    let n = read.usize();
    let tree = Tree::from_read_edges_p(n, Base::ZERO, read, |r| r.u32());

    let mut m = 0;
    let mut vid = vec![EXCLUDE; n];
    for i in 0..n {
        if tree.degree(i) > 1 {
            vid[i] = m;
            m += 1;
        }
    }

    let mut mat = SparseMatrix::new(m, m);
    let mut rhs = vec![ModNum::zero(); m];
    for i in 0..n {
        if tree.degree(i) > 1 {
            *mat.get_mut(vid[i], vid[i]) = ModNum::from(tree.degree(i) as u32);
            let mut sumw = ModNum::zero();
            for (j, &w) in tree.edges_adj(i) {
                if vid[j] != EXCLUDE {
                    *mat.get_mut(vid[i], vid[j]) = ModNum::from(-1);
                    *mat.get_mut(vid[j], vid[i]) = ModNum::from(-1);
                }
                sumw += ModNum::from(w);
            }
            rhs[vid[i]] = sumw;
        }
    }

    // TODO: Factor out sparse matrix solver.
    for i in (0..m).rev() {
        assert_ne!(mat[[i, i]], ModNum::zero());
        for ir in mat.col_indices(i).collect_vec().into_iter() {
            if ir >= i {
                continue;
            }
            let coeff = mat[[ir, i]] / mat[[i, i]];
            if coeff != ModNum::zero() {
                for (ic, v) in mat.row(i).collect_vec().into_iter() {
                    *mat.get_mut(ir, ic) -= v * coeff;
                }
                rhs[ir] = rhs[ir] - rhs[i] * coeff;
            }
        }
    }

    emitln!(write, rhs[0] / mat[[0, 0]]);
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
        3
        0 1 10
        0 2 20"), "15");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        0 1 3
        0 2 9
        0 3 27"), "13");
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        0 1 3
        0 5 7
        1 2 2
        1 3 1
        1 4 5
        5 6 8"), "400000019");
        assert_trimmed_eq!(&run_solver(solve, "\
        11
        1 0 6646
        2 0 8816
        3 2 9375
        4 2 5950
        5 1 8702
        6 2 2657
        7 2 885
        8 7 2660
        9 2 5369
        10 6 3798"), "153869806");
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        0 1 8
        0 2 24
        1 3 40
        1 4 16
        4 5 8"), "39");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
