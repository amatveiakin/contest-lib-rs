// UNFINISHED

use contest_lib_rs::array_2d::DynArray2D;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let [n, m, k] = read.usizes();
    let mut p: DynArray2D<Option<bool>> = DynArray2D::new(n - 1, m - 1);
    let mut failure = false;
    for _ in 0..k {
        let [x1, y1, x2, y2] = read.usizes();
        let [x1, y1, x2, y2] = [x1 - 1, y1 - 1, x2 - 1, y2 - 1];
        let xmin = x1.min(x2);
        let ymin = y1.min(y2);
        assert_eq!(xmin, x1);
        if p[(xmin, ymin)].is_some() {
            failure = true;
        }
        let v = y1 < y2;
        p[(xmin, ymin)] = Some(v);

        // ...
    }
    if failure {
        emitln!(write, "NO");
    } else {
        emitln!(write, "YES");
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 3 4 4
        // 1 1 2 2
        // 2 1 3 2
        // 1 4 2 3
        // 2 3 3 2
        // 2 7 2
        // 1 1 2 2
        // 1 2 2 1
        // 8 5 4
        // 1 2 2 1
        // 1 5 2 4
        // 7 1 8 2
        // 7 4 8 5
        // 8 5 4
        // 1 2 2 1
        // 1 5 2 4
        // 7 1 8 2
        // 7 5 8 4
        // "), "\
        // YES
        // NO
        // YES
        // NO");
    }
}
