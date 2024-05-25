// UNFINISHED

use contest_lib_rs::io::prelude::*;

fn set_bit(s: u32, i: u32, b: u32) -> u32 {
    if b == 0 {
        s & !(1 << i)
    } else {
        s | 1 << i
    }
}

fn f<W: std::io::Write>(n: u32, mut s: u32, i: u32, v: &[u32], write: &mut W) {
    if i == n - 1 {
        assert_eq!(v.len(), 1);
        if v[0] & 1 == 1 {
            emitln!(write, s);
        }
        return;
    }
    for b in [0, 1] {
        s = set_bit(s, i, b);
        let nvl = v.len() / 2;
        let mut nv = v[..nvl].to_vec();
        for j in 0..nvl {
            let mut vv = v[nvl + j];
            if b == 1 {
                vv <<= 1;
            }
            nv[j] &= vv;
        }
        f(n, s, i + 1, &nv, write);
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.u32();
    let mut v = read.vec_u32(1 << n - 1);
    v.reverse();
    v.push(1 << (n + 1) - 1);
    v.reverse();
    f(n, 0, 0, &v, write);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 15 15 15 15 15 15 12
        // "), "\
        // 4
        // 3
        // 5
        // 6
        // 7");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 63 63 63 63 6 63 63 63 63 63 63 5 63 63 63 63 63 63 8 63 63 63 63 2 63 63 63 63 63 63 63
        // "), "\
        // 1
        // 19");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
