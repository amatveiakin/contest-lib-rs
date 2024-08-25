use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_i32(n);
    let mut b = read.vec_i32(n);
    a.sort_unstable();
    b.sort_unstable();
    let mut maxmin = 0;
    for i in 0..n {
        let mut min = i32::MAX;
        for j in 0..n {
            min.relax_min((a[j] - b[(i + j) % n]).abs());
        }
        maxmin.relax_max(min);
    }
    emitln!(write, maxmin);
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
        3
        0 0 0
        1000000000 1000000000 1000000000
        5
        1 2 3 4 5
        1 2 3 4 5
        6
        0 0 0 100 100 100
        100 100 100 0 0 0
        7
        14 25 62 74 86 95 12
        51 62 71 72 92 20 84"), "\
        1000000000
        2
        100
        30");
    }
}
