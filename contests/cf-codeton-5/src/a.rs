use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let m = read.usize();
    let a = read.vec_i64(n);
    let b = read.vec_i64(m);
    let a_sum: i64 = a.iter().sum();
    let b_sum: i64 = b.iter().sum();
    let answer = match a_sum.cmp(&b_sum) {
        std::cmp::Ordering::Less => "Tenzing",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Tsondu",
    };
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        1 3
        9
        1 2 3
        2 3
        1 2
        1 1 1
        3 2
        1 2 3
        1 1
        3 3
        1 1 1
        2 2 2
        10 10
        1 2 3 3 2 2 1 1 2 2
        3 3 3 3 2 1 1 1 1 1
        10 10
        1 2 3 4 5 6 7 8 9 10
        6 7 8 9 10 11 1 1 1 1
        "), "\
        Tsondu
        Draw
        Tsondu
        Tenzing
        Draw
        Draw");
    }
}
