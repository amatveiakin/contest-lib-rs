use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    match n {
        1 => emitln!(write, 1),
        2 => emitln!(write, 1, 2),
        _ => {
            let mut a = vec![];
            a.push(2);
            let m = n - 3;
            for i in 0..(m/2) {
                a.push(i + 4);
            }
            a.push(1);
            for i in (m/2)..m {
                a.push(i + 4);
            }
            a.push(3);
            emitln!(write, a);
        }
    }
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 2
        // 1
        // 5
        // "), "\
        // 2 1
        // 1
        // 5 2 1 4 3");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        2
        1
        5
        "), "\
        1 2
        1
        2 4 1 5 3");
    }
}
