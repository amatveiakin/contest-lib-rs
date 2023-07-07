use contest_lib_rs::array_2d::CharArray2DReading;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let a = read.char_array2d(3, 3);
    for row in 0..3 {
        let p = a[(row, 0)];
        if p != '.' && a[(row, 1)] == p && a[(row, 2)] == p {
            emitln!(write, p.to_string());
            return;
        }
    }
    for col in 0..3 {
        let p = a[(0, col)];
        if p != '.' && a[(1, col)] == p && a[(2, col)] == p {
            emitln!(write, p.to_string());
            return;
        }
    }
    let p = a[(0, 0)];
    if p != '.' && a[(1, 1)] == p && a[(2, 2)] == p {
        emitln!(write, p.to_string());
        return;
    }
    let p = a[(0, 2)];
    if p != '.' && a[(1, 1)] == p && a[(2, 0)] == p {
        emitln!(write, p.to_string());
        return;
    }
    emitln!(write, "DRAW");
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
        5
        +X+
        OXO
        OX.
        O+.
        +OX
        X+O
        .XO
        OX.
        +++
        O.+
        X.O
        +..
        .++
        X.O
        +.."), "\
        X
        O
        +
        DRAW
        DRAW");
    }
}
