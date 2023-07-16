use contest_lib_rs::{io, emitln};

fn gcd_steps(mut a: i32, mut b: i32) -> i32 {
    let mut s = 0;
    while a != 0 {
        if b > 0 && a >= 2*b {
            // s += a / (2*b) * 3;
            a %= 2*b;
        } else {
            s += 1;
            (a, b) = (b, (a - b).abs());
        }
    }
    s % 3
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let va = read.vec_i32(n);
    let vb = read.vec_i32(n);
    let mut p = None;
    for i in 0..n {
        let a = va[i];
        let b = vb[i];
        if a == 0 && b == 0 {
            continue;
        }
        let q = gcd_steps(a, b);
        if let Some(p) = p {
            if p != q {
                emitln!(write, "NO");
                return;
            }
        }
        p = Some(q);
    }
    emitln!(write, "YES");
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
        // assert_eq!(gcd_steps(1, 2), 4);
        // assert_eq!(gcd_steps(2, 1), 3);

        assert_trimmed_eq!(&run_solver(solve, "\
        9
        4
        0 0 0 0
        1 2 3 4
        3
        1 2 3
        1 2 3
        2
        1 2
        2 1
        6
        100 23 53 11 56 32
        1245 31 12 6 6 6
        7
        1 2 3 4 5 6 7
        7 6 5 4 3 2 1
        3
        4 0 2
        4 0 2
        3
        2 5 2
        1 3 4
        2
        6 1
        4 2
        2
        0 0
        0 3
        "), "\
        YES
        YES
        NO
        NO
        YES
        YES
        NO
        YES
        YES");
    }
}
