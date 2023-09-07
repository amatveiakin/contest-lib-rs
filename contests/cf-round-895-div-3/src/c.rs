use contest_lib_rs::io::prelude::*;
use contest_lib_rs::primes::primes;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [l, r] = read.u32s();
    for v in l.max(r - 1)..=r {
        for p in primes() {
            if p * p > v {
                break;
            }
            if v % p == 0 {
                emitln!(write, p, v - p);
                return;
            }
        }
    }
    emitln!(write, -1);
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
        11
        11 15
        1 3
        18 19
        41 43
        777 777
        8000000 10000000
        2000 2023
        1791791 1791791
        1 4
        2 3
        9840769 9840769
        "), "\
        2 12
        -1
        2 16
        2 40
        3 774
        3 9999996
        2 2020
        -1
        2 2
        -1
        3137 9837632");

        // assert_trimmed_eq!(&run_solver(solve, "\
        // 11
        // 11 15
        // 1 3
        // 18 19
        // 41 43
        // 777 777
        // 8000000 10000000
        // 2000 2023
        // 1791791 1791791
        // 1 4
        // 2 3
        // 9840769 9840769
        // "), "\
        // 6 9
        // -1
        // 14 4
        // 36 6
        // 111 666
        // 4000000 5000000
        // 2009 7
        // -1
        // 2 2
        // -1
        // 6274 9834495");
    }
}
