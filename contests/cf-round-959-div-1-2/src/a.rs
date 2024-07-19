use contest_lib_rs::array_2d::{Array2D, Array2DReading};
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.u32s();
    let a = read.array2d(n as usize, m as usize);
    if n * m == 1 {
        emitln!(write, -1);
        return;
    }
    let b = a.map(|x: u32| x % (n * m) + 1);
    emitln!(write, b.to_string_separated(" "));
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
        // 5
        // 1 1
        // 1
        // 2 1
        // 2
        // 1
        // 1 5
        // 2 4 5 3 1
        // 2 4
        // 1 2 3 4
        // 5 6 7 8
        // 3 3
        // 4 2 1
        // 9 8 3
        // 6 7 5"), "\
        // -1
        // 1
        // 2
        // 4 5 3 1 2
        // 6 7 8 5
        // 2 3 4 1
        // 8 3 9
        // 7 5 6
        // 2 1 4 ");
    }
}
