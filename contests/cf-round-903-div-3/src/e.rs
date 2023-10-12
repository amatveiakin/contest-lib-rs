use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let mut r = vec![0; n + 1];
    for i in (0..n).rev() {
        let mut q = r[i + 1] + 1;
        if i + (a[i] as usize) < n {
            q.relax_min(r[i + a[i] as usize + 1]);
        }
        r[i] = q;
    }
    emitln!(write, r[0]);
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
        7
        3 3 4 5 2 6 1
        4
        5 6 3 2
        6
        3 4 1 6 7 7
        3
        1 4 3
        5
        1 2 3 4 5
        5
        1 2 3 1 2
        5
        4 5 5 1 5
        "), "\
        0
        4
        1
        1
        2
        1
        0");
    }
}
