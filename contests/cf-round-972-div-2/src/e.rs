// UNFINISHED

use contest_lib_rs::array_2d::{Array2DReading, DynArray2D};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [l, n, m] = read.usizes();
    let a = read.vec_u32(l);
    let b: DynArray2D<u32> = read.array2d(n, m);
    let mut fr = vec![-1i64; n];
    for &av in a.iter().rev() {
        let mut nfr = vec![-1i64; n];
        let mut maxcol = -1;
        for row in (0..n).rev() {
            let startcol = fr[row].max(0) as usize;
            for col in (startcol..m).rev() {
                if b[[row, col]] == av {
                    // println!("MAXCOL: {} {} {}", row, col, av);
                    maxcol.relax_max(col as i64);
                    break;
                }
            }
            nfr[row] = maxcol;
        }
        fr = nfr;
        // println!("{:?}", fr);
    }
    let winner = if fr[0] > -1 { 'T' } else { 'N' };
    emitln!(write, winner);
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
        // assert_trimmed_eq!(&run_solver(solve_case, "\
        // 3 3 3
        // 1 2 3
        // 1 7 7
        // 7 2 3
        // 7 7 7"), "N");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        3 3 3
        1 2 3
        1 7 7
        7 2 7
        7 7 3"), "T");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        2 2 3
        1 2
        1 3 5
        4 5 2
        2 2 4
        1 2
        1 1 3 2
        4 2 5 1
        2 4 2
        1 2
        3 4
        5 5
        5 5
        5 5"), "\
        N
        T
        N");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
