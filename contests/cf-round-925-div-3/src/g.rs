use contest_lib_rs::combinatorics_mod::num_combinations_mod;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::num::RingNumber;

type ModNum = ModNumber<998244353>;

fn gr(n: i32, k: i32) -> ModNum {
    assert!(k > 0);
    num_combinations_mod(n + k - 1, k - 1)
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [c1, c2, c3, c4] = read.i32s();

    if c1 == 0 && c2 == 0 {
        if c3 > 0 && c4 > 0 {
            emitln!(write, 0);
        } else {
            emitln!(write, 1);
        }
        return;
    }
    let ans = match c1.abs_diff(c2) {
        0 => {
            let k = (c1 + c2) / 2;
            gr(c3, k) * gr(c4, k+1) + gr(c3, k+1) * gr(c4, k)
        }
        1 => {
            let k = (c1 + c2 + 1) / 2;
            gr(c3, k) * gr(c4, k)
        }
        _ => ModNum::zero()
    };
    emitln!(write, ans);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        11
        1 1 1 1
        1 2 5 10
        4 6 100 200
        900000 900000 900000 900000
        0 0 0 0
        0 0 566 239
        1 0 0 0
        100 0 100 0
        0 0 0 4
        5 5 0 2
        5 4 0 5
        "), "\
        4
        66
        0
        794100779
        1
        0
        1
        0
        1
        36
        126");
    }
}
