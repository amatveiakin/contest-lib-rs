use contest_lib_rs::{io, emitln};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let p = read.vec_i32(n);
    let mut k = None;
    for i in 0..n {
        let v = (p[i] - (i + 1) as i32).abs();
        k = match k {
            None => Some(v),
            Some(u) => Some(gcd(u, v)),
        }
    }
    emitln!(write, k.unwrap());
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
        7
        3
        3 1 2
        4
        3 4 1 2
        7
        4 2 6 7 5 3 1
        9
        1 6 7 4 9 2 3 8 5
        6
        1 5 3 4 2 6
        10
        3 10 5 2 9 6 7 8 1 4
        11
        1 11 6 4 8 3 7 5 9 10 2
        "), "\
        1
        2
        3
        4
        3
        2
        3");
    }
}
