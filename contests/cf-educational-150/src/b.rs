use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let q = read.usize();
    let x = read.vec_i32(q);
    let mut first = -1;
    let mut last = -1;
    let mut dec = 0;
    let mut answer = String::new();
    for v in x {
        if first == -1 {
            first = v;
            last = v;
            answer.push('1');
            continue;
        }
        if dec == 1 {
            if v >= last && v <= first {
                last = v;
                answer.push('1');
            } else {
                answer.push('0');
            }
        } else {
            if v >= last {
                last = v;
                answer.push('1');
            } else if v <= first {
                dec += 1;
                last = v;
                answer.push('1');
            } else {
                answer.push('0');
            }
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        9
        3 7 7 9 2 4 6 3 4
        5
        1 1 1 1 1
        5
        3 2 1 2 3
        "), "\
        111110010
        11111
        11011");
        assert_trimmed_eq!(&run_solver(solve, "1  3  1 3 2"), "110");
        assert_trimmed_eq!(&run_solver(solve, "1  3  1 3 1"), "111");
    }
}
