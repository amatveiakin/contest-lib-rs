use contest_lib_rs::{io, emitln};

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a > b {
        gcd(a % b, b)
    } else {
        gcd(a, b % a)
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u64(n);
    let mut answer = 0;
    for i in 0..(n / 2) {
        let x = a[i].abs_diff(a[n - i - 1]);
        answer = gcd(answer, x);
    }
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        2
        1 2
        8
        3 0 1 2 0 3 2 1
        1
        0
        3
        100 1 1000000000
        "), "\
        1
        2
        0
        999999900");
    }
}
