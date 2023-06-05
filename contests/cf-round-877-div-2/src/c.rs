use contest_lib_rs::{io, emitln, emit};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let mut n = read.u32();
    let mut m = read.u32();
    if n == 4 && m == 4 {
        emitln!(write, 1, 2, 3, 4);
        emitln!(write, 5, 6, 7, 8);
        emitln!(write, 9, 10, 11, 12);
        emitln!(write, 13, 14, 15, 16);
        return;
    }
    let mut tr = false;
    if n > m {
        std::mem::swap(&mut n, &mut m);
        tr = true;
    }
    assert!(m >= 5);
    let mut a = vec![0; (n * m) as usize];
    let mut row_start = 0;
    for i in 0..m {
        for j in 0..n {
            a[(i * n + j) as usize] = row_start * n + j + 1;
        }
        row_start += 2;
        if row_start >= m {
            row_start = 1;
        }
    }
    if !tr {
        for j in 0..n {
            for i in 0..m {
                emit!(write, a[(i * n + j) as usize]);
            }
            emitln!(write, "");
        }
    } else {
        for i in 0..m {
            for j in 0..n {
                emit!(write, a[(i * n + j) as usize]);
            }
            emitln!(write, "");
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "2  4 5  5 4"), "");
    }
}
