use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u64(n);
    let mut a = a.into_iter().enumerate().map(|(i, x)| (x, i)).collect_vec();
    a.sort();
    let mut p = 0;
    let mut t = 0;
    let mut ans = vec![0; n];
    for i in 0..n {
        assert!(p >= i);
        if p == i {
            t += a[i].0;
            p += 1;
        }
        while p < n && a[p].0 <= t {
            t += a[p].0;
            p += 1;
        }
        ans[a[i].1] = p - 1;
    }
    emitln!(write, ans);
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
        4
        5
        20 5 1 4 2
        3
        1434 7 1442
        1
        1
        5
        999999999 999999999 999999999 1000000000 1000000000
        "), "\
        4 3 0 3 1
        1 0 2
        0
        4 4 4 4 4 ");
    }
}
