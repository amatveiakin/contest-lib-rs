// TODO: Add sparse array for queries like "`and` on a segment".

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::segment_tree::new_homogenous_tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let mut t = new_homogenous_tree(&a, u32::MAX, |&a, &b, _| a & b);
    let q = read.usize();
    for _ in 0..q {
        let l0 = read.u32().from1b();
        let k = read.u32();
        if t.get(l0) < k {
            emit!(write, -1);
            continue;
        }
        let mut l = l0;
        let mut r = n as u32 - 1;
        while l < r {
            let m = (l + r + 1) / 2;
            if t.get(l0..=m) < k {
                r = m - 1;
            } else {
                l = m;
            }
        }
        emit!(write, r.to1b());
    }
    emitln!(write);
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
        3
        5
        15 14 17 42 34
        3
        1 7
        2 15
        4 5
        5
        7 5 3 1 7
        4
        1 7
        5 7
        2 3
        2 2
        7
        19 20 15 12 21 7 11
        4
        1 15
        4 4
        7 12
        5 7
        "), "\
        2 -1 5
        1 5 2 2
        2 6 -1 5 ");
        // assert_trimmed_eq!(&run_solver(solve_case, "\
        // 7
        // 19 20 15 12 21 7 11
        // 4
        // 1 15
        // 4 4
        // 7 12
        // 5 7
        // "), "2 6 -1 5 ");
    }
}
