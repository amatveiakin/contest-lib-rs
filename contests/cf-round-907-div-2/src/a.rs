use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_is_sorted::IterutilsIsSorted;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    for m in 0.. {
        let m2 = 2usize.pow(m);
        if m2 > n {
            break;
        }
        if !a[m2.min(n)..(m2 * 2).min(n)].iter().issorted() {
            emitln!(write, "NO");
            return;
        }
    }
    emitln!(write, "YES");
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
        8
        5
        1 2 3 4 5
        5
        6 5 3 4 4
        9
        6 5 5 7 5 6 6 8 7
        4
        4 3 2 1
        6
        2 2 4 5 3 2
        8
        1 3 17 19 27 57 179 13
        5
        3 17 57 179 92
        10
        1 2 3 4 0 6 7 8 9 10
        "), "\
        YES
        YES
        YES
        NO
        NO
        NO
        YES
        YES");
    }
}
