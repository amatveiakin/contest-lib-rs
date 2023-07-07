use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut answer = 0;
    for _ in 0..n {
        let a = read.i32();
        let b = read.i32();
        if a > b {
            answer += 1;
        }
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
        3
        4 3
        3 1
        1 2
        4
        9 2
        5 2
        7 7
        3 4
        5
        11 7
        5 10
        12 9
        3 2
        1 5
        3
        5 6
        4 5
        7 7"), "\
        2
        2
        3
        0");
    }
}
