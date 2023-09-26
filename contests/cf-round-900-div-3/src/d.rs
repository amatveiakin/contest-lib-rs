use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::sort_array::sort_array;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let s = read.word_as_chars();
    assert_eq!(s.len(), n);
    let ls = read.vec_usize(k).from1b();
    let rs = read.vec_usize(k).from1b();
    let q = read.usize();
    let xs = read.vec_usize(q).from1b();

    let mut to_lr = vec![(0, 0); n];
    for i in 0..k {
        let l = ls[i];
        let r = rs[i];
        for j in l..=r {
            to_lr[j] = (l, r);
        }
    }

    let mut inv = vec![false; n];
    for x in xs {
        let (l, r) = to_lr[x];
        let [a, b] = sort_array([x, l + r - x]);
        inv[a] = !inv[a];
        inv[b] = !inv[b];
    }

    let mut prev_lr = None;
    let mut is_inv = false;
    let mut ans = vec![];
    for x in 0..n {
        let lr = to_lr[x];
        let (l, r) = lr;
        if prev_lr != Some(lr) {
            is_inv = false;
            prev_lr = Some(lr);
        }
        let y = l + r - x;
        let first_half = x <= (l + r) / 2;
        if first_half {
            is_inv ^= inv[x];
        }
        ans.push(if is_inv { s[y] } else { s[x] });
        if !first_half {
            is_inv ^= inv[x];
        }
    }
    emitln!(write, ans.into_iter().join(""))
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        4 2
        abcd
        1 3
        2 4
        2
        1 3
        5 3
        abcde
        1 2 3
        1 2 5
        3
        1 2 3
        3 1
        gaf
        1
        3
        2
        2 2
        10 1
        aghcdegdij
        1
        10
        5
        1 2 3 4 2
        1 1
        a
        1
        1
        1
        1
        "), "\
        badc
        abedc
        gaf
        jihgedcdga
        a");
    }
}
