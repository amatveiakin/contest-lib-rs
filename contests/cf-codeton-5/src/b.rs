use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let x = read.i32();
    let mut k = 0;
    for _ in 0..3 {
        let a = read.vec_i32(n);
        for &v in &a {
            let kk = k | v;
            if kk | x == x {
                k = kk;
            } else {
                break;
            }
        }
    }
    if k == x {
        emitln!(write, "Yes");
    } else {
        emitln!(write, "No");
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        5 7
        1 2 3 4 5
        5 4 3 2 1
        1 3 5 7 9
        5 2
        3 2 3 4 5
        5 4 3 2 1
        3 3 5 7 9
        3 0
        1 2 3
        3 2 1
        2 2 2
        "), "\
        Yes
        No
        Yes");
    }
}
