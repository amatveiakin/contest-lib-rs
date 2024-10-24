use std::cmp;

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let s = (0..n).map(|_| {
        let si = read.word_as_digits();
        (si.iter().filter(|&&x| x == 0).count(), si.iter().filter(|&&x| x == 1).count())
    }).collect_vec();
    let mut maj0 = 0;
    let mut maj1 = 0;
    let mut draw = 0;
    for &(a, b) in s.iter() {
        if a > b {
            maj0 += 1;
        } else if b > a {
            maj1 += 1;
        } else {
            draw += 1;
        }
    }
    let sum0 = s.iter().map(|(a, b)| a).sum::<usize>();
    let sum1 = s.iter().map(|(a, b)| b).sum::<usize>();
    let ans = if sum0 == 0 || sum1 == 0 {
        0
    } else if maj0 == n {
        let min_diff = s.iter().map(|(a, b)| a - b).min().unwrap();
        sum1 + min_diff
    } else if maj1 == n {
        let min_diff = s.iter().map(|(a, b)| b - a).min().unwrap();
        sum0 + min_diff
    } else {
        s.iter().map(|(a, b)| cmp::min(a, b)).sum::<usize>()
    };
    emitln!(write, ans);
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
        11101101
        00
        10001
        10"), "5");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        101010
        010101
        "), "6");
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        0000
        11
        0
        00000000
        1"), "0");

        assert_trimmed_eq!(&run_solver(solve, "\
        3
        000001111
        0000111
        00011"), "10");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        000011111
        0001111
        00111"), "10");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        010101
        0101"), "5");

        assert_trimmed_eq!(&run_solver(solve, "\
        3
        1111
        0011
        0000"), "2");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        1111
        00011
        0000"), "2");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        1111
        00111
        0000"), "2");

        assert_trimmed_eq!(&run_solver(solve, "\
        2
        111
        1111"), "0");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        000
        0000"), "0");

        assert_trimmed_eq!(&run_solver(solve, "\
        2
        000
        000001"), "4");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        000
        000011"), "4");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        111
        111110"), "4");
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        111
        111100"), "4");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
