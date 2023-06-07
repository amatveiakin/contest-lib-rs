use std::collections::HashMap;

use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let s = read.word();
    let mut symbols = HashMap::new();
    for c in s.chars() {
        *symbols.entry(c).or_insert(0) += 1;
    }
    let answer = symbols.values().filter(|&&v| v >= 2).count() >= 2;
    emitln!(write, if answer { "YES" } else { "NO" });
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
        3
        codedoc
        gg
        aabaa
        "), "\
        YES
        NO
        NO");
    }
}
