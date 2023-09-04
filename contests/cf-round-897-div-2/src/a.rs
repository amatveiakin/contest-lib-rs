use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let mut s = a.iter().copied().enumerate().collect::<Vec<_>>();
    s.sort_by_key(|&(_, x)| -x);
    let mut b = vec![0; n];
    for (bp, (ap, _)) in s.iter().copied().enumerate() {
        b[ap] = bp + 1;
    }
    emitln!(write, b);
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 1
        // 100000
        // 2
        // 1 1
        // 3
        // 10 3 3
        // "), "\
        // 1
        // 2 1
        // 1 3 2 ");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        4
        2 6 4 8
        "), "\
        4 2 3 1 ");
    }
}
