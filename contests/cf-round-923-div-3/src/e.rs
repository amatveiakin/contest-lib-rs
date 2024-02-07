use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();

    let mut a = vec![0; n];
    let mut l = 0;
    let mut r = n - 1;
    for start in 0..k {
        for i in (start..n).step_by(k) {
            if start % 2 == 0 {
                a[i] = l;
                l += 1;
            } else {
                a[i] = r;
                r -= 1;
            }
        }
    }
    assert_eq!(l, r + 1);

    emitln!(write, a.to1b());
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
        // 2 2
        // 3 2
        // 10 4
        // 13 4
        // 7 4
        // "), "\
        // 1 2
        // 3 1 2
        // 8 3 10 1 7 4 9 2 6 5
        // 13 4 9 1 12 5 8 2 11 6 7 3 10
        // 1 7 3 5 2 6 4
        // ");
    }
}
