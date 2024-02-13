use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i32();
    let mut a = read.vec_i32(n as usize);
    a.sort_unstable();
    a.dedup();
    let mut ans = 0;
    for (i, &x) in a.iter().enumerate() {
        let j = match a.binary_search(&(x - n + 1)) {
            Ok(p) | Err(p) => p,
        };
        ans.relax_max(i - j + 1);
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
        7
        2
        1 2
        4
        7 1 4 1
        3
        103 102 104
        5
        1 101 1 100 1
        5
        1 10 100 1000 1
        2
        3 1
        3
        1000000000 999999997 999999999
        "), "\
        2
        2
        3
        2
        1
        1
        2");
    }
}
