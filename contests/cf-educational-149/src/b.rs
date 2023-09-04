use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let s = read.word();
    let mut len = 0;
    let mut max_len = 0;
    let mut last = None;
    for c in s.chars() {
        if last == Some(c) {
            len += 1;
        } else {
            max_len = max_len.max(len);
            len = 1;
        }
        last = Some(c);
    }
    max_len = max_len.max(len);
    emitln!(write, max_len + 1);
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
        4
        4
        <<>>
        4
        >><<
        5
        >>>>>
        7
        <><><><"), "\
        3
        3
        6
        2");
    }
}
