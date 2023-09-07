use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let s = read.word().chars().map(|ch| ch.to_digit(10).unwrap()).collect::<Vec<_>>();
    assert_eq!(s.len(), n);
    let q = read.usize();

    let mut xor = 0;
    for i in 0..n {
        if s[i] == 1 {
            xor ^= a[i];
        }
    }

    let mut part = vec![0; n + 1];
    for i in 0..n {
        part[i + 1] = part[i] ^ a[i];
    }

    for _ in 0..q {
        let cmd = read.u32();
        match cmd {
            1 => {
                let [l, r] = read.usizes();
                xor ^= part[l - 1] ^ part[r];
            }
            2 => {
                let g = read.u32();
                let ans = match g {
                    0 => xor ^ part[n],
                    1 => xor,
                    _ => unreachable!(),
                };
                emit!(write, ans);
            }
            _ => unreachable!(),
        }
    }
    emitln!(write, "");
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
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
        5
        1 2 3 4 5
        01000
        7
        2 0
        2 1
        1 2 4
        2 0
        2 1
        1 1 3
        2 1
        6
        12 12 14 14 5 5
        001001
        3
        2 1
        1 2 4
        2 1
        4
        7 7 7 777
        1111
        3
        2 0
        1 2 3
        2 0
        2
        1000000000 996179179
        11
        1
        2 1
        5
        1 42 20 47 7
        00011
        5
        1 3 4
        1 1 1
        1 3 4
        1 2 4
        2 0
        "), "\
        3 2 6 7 7
        11 7
        0 0
        16430827
        47 ");
    }
}
