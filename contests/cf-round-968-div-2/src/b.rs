use std::cmp;

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_u32(n);
    a.sort_unstable();
    if n % 2 == 0 {
        emitln!(write, cmp::max(a[n/2 - 1], a[n/2]));
    } else {
        emitln!(write, a[n/2]);
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
        5
        2
        1 2
        3
        1 1 2
        3
        1 2 3
        5
        3 1 2 2 3
        10
        10 2 5 2 7 9 2 5 10 7"), "\
        2
        1
        2
        2
        7");
    }
}
