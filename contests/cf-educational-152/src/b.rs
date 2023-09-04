use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let k = read.i32();
    let a = read.vec_i32(n);
    let mut a = a.into_iter().enumerate().map(|(i, x)| (-(x - 1).rem_euclid(k), i + 1)).collect::<Vec<_>>();
    a.sort();
    let p = a.into_iter().map(|(_, i)| i).collect::<Vec<_>>();
    emitln!(write, p);
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
        3
        3 2
        1 2 3
        2 3
        1 1
        4 3
        2 8 3 5"), "\
        2 1 3
        1 2
        3 1 2 4 ");
    }
}
