use std::cmp;

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

    let mut b: u64 = 0;
    for ls in lss.iter() {
        let mex = get_mex(ls.iter().copied());
        let mut ls = ls.clone();
        ls.push(mex);
        let mex2 = get_mex(ls.iter().copied());
        b.relax_max(mex2);
    }

    let mut ans: u64 = (cmp::min(m, b) + 1) * b;
    if m > b {
        ans += m * (m+1) / 2 - b * (b+1) / 2;
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
        20
        1281
        6
        6556785365
        1842836177961");
        // assert_trimmed_eq!(&run_solver(solve_case, "\
        // 3 4
        // 2 0 2
        // 3 2 3 3
        // 4 7 0 1 5"), "16");
    }
}
