use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);

    let mut prefix_lt = vec![0; n];
    let mut prefix_eq = vec![0; n];
    let mut prefix_gt = vec![0; n];
    let mut suffix_lt = vec![0; n];
    let mut suffix_eq = vec![0; n];
    let mut suffix_gt = vec![0; n];

    for i in 1..n {
        prefix_lt[i] = prefix_lt[i - 1];
        prefix_eq[i] = prefix_eq[i - 1];
        prefix_gt[i] = prefix_gt[i - 1];
        match a[i].cmp(&a[i - 1]) {
            std::cmp::Ordering::Less => prefix_lt[i] += 1,
            std::cmp::Ordering::Equal => prefix_eq[i] += 1,
            std::cmp::Ordering::Greater => prefix_gt[i] += 1,
        }
    }
    for i in (0..(n - 1)).rev() {
        suffix_lt[i] = suffix_lt[i + 1];
        suffix_eq[i] = suffix_eq[i + 1];
        suffix_gt[i] = suffix_gt[i + 1];
        match a[i + 1].cmp(&a[i]) {
            std::cmp::Ordering::Less => suffix_lt[i] += 1,
            std::cmp::Ordering::Equal => suffix_eq[i] += 1,
            std::cmp::Ordering::Greater => suffix_gt[i] += 1,
        }
    }

    let mut answer = prefix_lt[n - 1] + prefix_eq[n - 1];
    for i in 1..n {
        answer.relax_min(1 + prefix_gt[i - 1] + prefix_eq[i - 1] + suffix_lt[i] + suffix_eq[i]);
    }
    emitln!(write, answer);
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
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
3
5
1 1 2 2 2
6
5 4 3 2 5 1
3
1 2 3
"), "\
3
2
0");
        assert_trimmed_eq!(&run_solver(solve_case, "5  1 1 1 1 1"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "3  3 2 1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "3  2 1 2"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "3  2 1 1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "6  6 5 4 3 2 1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "6  3 2 1 1 2 3"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "4  2 1 1 2"), "1");
        // assert_trimmed_eq!(&run_solver(solve_case, ""), "");
    }
}
