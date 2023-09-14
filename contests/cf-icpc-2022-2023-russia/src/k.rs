use contest_lib_rs::array_2d::{Array2DReading, DynArray2D};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a: DynArray2D<u64> = read.array2d(n, n);
    let mut min = u64::MAX;
    for i in 0..n {
        min.relax_min(a[[n - i - 1, i]]);
    }
    let ans = a.iter_enumerated().map(|(_, &v)| v).sum::<u64>() - min;
    emitln!(write, ans);
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
        2
        1 2
        3 4
        "), "8");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        10 10 10
        10 0 10
        10 10 10
        "), "80");
    }
}
