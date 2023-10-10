use contest_lib_rs::relax::Relax;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let mut next_color = vec![None; n + 1];
    let mut m = vec![0; n + 1];
    for i in (0..n).rev() {
        let c = a[i];
        m[i] = m[i + 1];
        if let Some(j) = next_color[c as usize] {
            let suffix_incl = m[j];
            let suffix_excl = m[j + 1];
            m[i].relax_max((j - i + 1) + suffix_incl - 1);
            m[i].relax_max((j - i + 1) + suffix_excl);
        }
        next_color[c as usize] = Some(i);
    }
    let answer = m.iter().max().unwrap();
    emitln!(write, answer);
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        2
        5
        1 2 2 3 3
        4
        1 2 1 2
        "), "\
        4
        3");
        assert_trimmed_eq!(&run_solver(solve_case, "9  1 2 1 3 4 5 6 2 6"), "7");
        assert_trimmed_eq!(&run_solver(solve_case, "23  1 4 5 6 7 8 2 1 9 10 11 12 13 14 15 3 2 16 17 18 19 20 3"), "16");
        assert_trimmed_eq!(&run_solver(solve_case, "1  1"), "0");
        assert_trimmed_eq!(&run_solver(solve_case, "2  1 1"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "3  1 1 1"), "3");
        assert_trimmed_eq!(&run_solver(solve_case, "4  1 1 1 1"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "5  1 1 1 1 1"), "5");
        assert_trimmed_eq!(&run_solver(solve_case, "3  1 2 3"), "0");
        assert_trimmed_eq!(&run_solver(solve_case, "5  1 1 2 2 1"), "5");
        assert_trimmed_eq!(&run_solver(solve_case, "5  1 2 2 1 1"), "5");
        assert_trimmed_eq!(&run_solver(solve_case, "6  1 1 1 2 2 2"), "6");
        assert_trimmed_eq!(&run_solver(solve_case, "6  1 1 2 1 2 2"), "6");
        // assert_trimmed_eq!(&run_solver(solve_case, ""), "");
    }
}
