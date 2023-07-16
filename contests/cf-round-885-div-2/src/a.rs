use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let [n, m] = read.i32s();
    let k = read.usize();
    let [x, y] = read.i32s();
    let mut run = true;
    for _ in 0..k {
        let [xi, yi] = read.i32s();
        if (x + y) % 2 == (xi + yi) % 2 {
            run = false;
        }
    }
    emitln!(write, if run { "YES" } else { "NO" });
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
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        2 2 1
        1 1
        1 2
        2 2 2
        1 1
        2 2
        2 2
        1 2 1
        1 1
        1 2
        5 5 4
        3 3
        1 1
        1 5
        5 1
        5 5
        2 2 2
        1 1
        2 1
        1 2
        3 4 1
        1 2
        3 3
        "), "\
        YES
        NO
        YES
        NO
        YES
        YES
        ");
    }
}
