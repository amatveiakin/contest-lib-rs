use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::iterutils_zip_eq::IterutilsZipEq;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let b = read.vec_u32(n);
    let mut ab = a.into_iter().zip_eq(b.into_iter()).collect_vec();
    ab.sort();
    let (a, b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();
    emitln!(write, a);
    emitln!(write, b);
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
        // 3
        // 5
        // 1 2 3 4 5
        // 5 4 3 2 1
        // 3
        // 3 1 2
        // 3 1 2
        // 6
        // 2 5 6 1 3 4
        // 1 5 3 6 2 4
        // "), "\
        // 3 2 5 1 4
        // 3 4 1 5 2
        // 1 2 3
        // 1 2 3
        // 2 3 4 6 5 1
        // 1 2 4 3 5 6");
    }
}
