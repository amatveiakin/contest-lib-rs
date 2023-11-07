use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::iterutils_windows::IterutilsWindows;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_i32(2 * n);
    a.sort();
    let ps = (0..n).map(|i| (a[i], a[i + n])).collect_vec();
    let l: i32 = ps.iter().array_windows()
        .map(|[(x0, y0), (x1, y1)]| (x1 - x0).abs() + (y1 - y0).abs())
        .sum();
    emitln!(write, l);
    for (x, y) in ps {
        emitln!(write, x, y);
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 2
        // 2
        // 15 1 10 5
        // 3
        // 10 30 20 20 30 10
        // "), "\
        // 9
        // 10 1
        // 15 5
        // 20
        // 20 20
        // 10 30
        // 10 30");
    }
}
