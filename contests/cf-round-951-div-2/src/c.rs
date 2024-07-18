use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let k = read.vec_u64(n);

    let b = 232_792_560_u64;
    let s = k.iter().copied().map(|x| b / x + 1).collect_vec();
    if s.iter().sum::<u64>() <= b {
        emitln!(write, s);
    } else {
        emitln!(write, -1);
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
        // 6
        // 3
        // 3 2 7
        // 2
        // 3 3
        // 5
        // 5 5 5 5 5
        // 6
        // 7 9 3 17 9 13
        // 3
        // 6 3 2
        // 5
        // 9 4 6 8 3
        // "), "\
        // 27 41 12
        // 1 1
        // -1
        // 1989 1547 4641 819 1547 1071
        // -1
        // 8 18 12 9 24");
    }
}
