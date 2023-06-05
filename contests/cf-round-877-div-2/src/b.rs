use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.i32();
    let a = read.vec_i32(n as usize);
    let p1 = a.iter().position(|&v| v == 1).unwrap() + 1;
    let p2 = a.iter().position(|&v| v == 2).unwrap() + 1;
    let pn = a.iter().position(|&v| v == n).unwrap() + 1;
    if (pn < p1) != (pn < p2) {
        emitln!(write, 1, 1);
    } else if (p1 < p2) != (p1 < pn) {
        emitln!(write, pn, p1);
    } else {
        emitln!(write, pn, p2);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 8
        // 3
        // 1 2 3
        // 3
        // 1 3 2
        // 5
        // 1 3 2 5 4
        // 6
        // 4 5 6 1 2 3
        // 9
        // 8 7 6 3 2 1 4 5 9
        // 10
        // 7 10 5 1 9 8 3 2 6 4
        // 10
        // 8 5 10 9 2 1 3 4 6 7
        // 10
        // 2 3 5 7 10 1 8 6 4 9
        // "), "\
        // 2 3
        // 1 1
        // 5 2
        // 1 4
        // 9 5
        // 8 8
        // 6 10
        // 5 4");
    }
}
