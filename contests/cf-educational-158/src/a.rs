use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_windows::IterutilsWindows;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let x = read.u32();
    let a = read.vec_u32(n);
    let mut ans = a[0];
    for [x, y] in a.iter().array_windows() {
        ans.relax_max(y - x);
    }
    ans.relax_max((x - a.last().unwrap()) * 2);
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
        3
        3 7
        1 2 5
        3 6
        1 2 5
        1 10
        7
        "), "\
        4
        3
        7");
    }
}
