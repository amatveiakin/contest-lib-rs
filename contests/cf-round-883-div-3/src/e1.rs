use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.i64();
    for k in 2..n {
        if k * k > n {
            break;
        }
        let mut a = 1 + k;
        let mut kk = k;
        loop {
            kk *= k;
            a += kk;
            if a == n {
                emitln!(write, "YES");
                return;
            } else if a > n {
                break;
            }
        }
    }
    emitln!(write, "NO");
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
        9
        1
        2
        3
        6
        13
        15
        255
        10101
        1000000
        "), "\
        NO
        NO
        NO
        NO
        YES
        YES
        YES
        YES
        NO");
    }
}
