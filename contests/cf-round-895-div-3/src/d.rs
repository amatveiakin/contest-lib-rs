use contest_lib_rs::io::prelude::*;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, x, y] = read.i64s();
    let cxy = n / lcm(x, y);
    let cx = n / x - cxy;
    let cy = n / y - cxy;
    let scx = cx * (cx + 1) / 2 + cx * (n - cx);
    let scy = cy * (cy + 1) / 2;
    let sc = scx - scy;
    emitln!(write, sc);
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
        8
        7 2 3
        12 6 3
        9 1 9
        2 2 2
        100 20 50
        24 4 6
        1000000000 5575 25450
        4 4 1
        "), "\
        12
        -3
        44
        0
        393
        87
        179179179436104
        -6
        ");
    }
}
