use contest_lib_rs::{io, emitln, mod_ring::ModNumber};

type ModNum = ModNumber::<1_000_000_007>;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_i32(n);
    let mut b = read.vec_i32(n);
    a.sort();
    b.sort();
    let mut r = 0;
    let mut answer = ModNum::from(1);
    for l in 0..n {
        if r < n {
            let bs = &b[r..];
            r += bs.iter().copied().position(|x| x >= a[l]).unwrap_or(bs.len());
        }
        let m = r as i32 - l as i32;
        if m == 0 {
            emitln!(write, 0);
            return;
        }
        answer *= ModNum::from(m);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        6
        9 6 8 4 5 2
        4 1 5 6 3 1
        3
        4 3 2
        3 4 9
        1
        2
        1
        3
        2 3 4
        1 3 3
        12
        2 3 7 10 23 28 29 50 69 135 420 1000
        1 1 2 3 5 8 13 21 34 55 89 144
        "), "\
        32
        0
        1
        0
        13824");
    }
}
