use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::combinatorics_mod::{num_combinations_mod, num_permutations_mod};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_windows::IterutilsWindows;
use contest_lib_rs::mod_ring::ModNumber;

type ModNum = ModNumber<1000000007>;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i32();
    let [m1, m2] = read.usizes();
    let p = read.vec_i32(m1).from1b();
    let q = read.vec_i32(m2).from1b();
    if *p.first().unwrap() != 0 || *q.last().unwrap() != n - 1 {
        emitln!(write, 0);
        return;
    }
    if *p.last().unwrap() != *q.first().unwrap() {
        emitln!(write, 0);
        return;
    }
    let maxpos = *p.last().unwrap();
    let mut ans: ModNum = num_combinations_mod(n - 1, maxpos);
    for [x, y] in p.into_iter().array_windows() {
        ans *= num_permutations_mod(y - 1, y - x - 1);
    }
    for [x, y] in q.into_iter().array_windows() {
        ans *= num_permutations_mod(n - x - 2, y - x - 1);
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
        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 1 2
        1
        1 3
        "), "0");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 1 2
        1
        1 4
        "), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 2 1
        1 4
        4
        "), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        6 2 2
        1 3
        3 6
        "), "20");
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        1 1 1
        1
        1
        4 2 3
        1 2
        2 3 4
        3 3 1
        1 2 3
        3
        5 3 4
        1 2 3
        2 3 4 5
        20 5 4
        1 2 3 4 12
        12 13 18 20
        6 2 3
        1 3
        3 4 6
        "), "\
        1
        3
        1
        0
        317580808
        10");
    }
}
