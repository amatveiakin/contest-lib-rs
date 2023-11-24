use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_windows::IterutilsWindows;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let c = read.vec_u64(n);
    let ans = c.iter().sum::<u64>()
        - c.iter().array_windows().map(|[x, y]| x.min(y)).sum::<u64>()
        - 1;
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
        4
        1 2 2 1
        5
        1 0 1 0 1
        5
        5 4 3 2 1
        1
        12
        "), "\
        1
        2
        4
        11");
    }
}
