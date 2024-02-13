use std::collections::HashSet;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::divisors::divisors;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, x] = read.u64s().from1b();
    let mut div = HashSet::new();
    for d in divisors(n - x) {
        if d % 2 == 1 {
            continue;
        }
        let k = (d + 2) / 2;
        if n % d < k && x == n % d {
            div.insert(d);
        }
    }
    for d in divisors(n + x) {
        if d % 2 == 1 {
            continue;
        }
        let k = (d + 2) / 2;
        if n % d >= k && x == d - n % d {
            div.insert(d);
        }
    }
    emitln!(write, div.len());
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        10 2
        3 1
        76 4
        100 99
        1000000000 500000000
        "), "\
        4
        1
        9
        0
        1");
    }
}
