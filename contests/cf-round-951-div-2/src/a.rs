use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_windows::IterutilsWindows;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    emitln!(write, a.iter().array_windows().map(|[a, b]| a.max(b)).min().unwrap() - 1);
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
        4
        2 4 1 7
        5
        1 2 3 4 5
        2
        1 1
        3
        37 8 16
        5
        10 10 10 10 9
        10
        3 12 9 5 2 3 2 9 8 2
        "), "\
        3
        1
        0
        15
        9
        2");
    }
}
