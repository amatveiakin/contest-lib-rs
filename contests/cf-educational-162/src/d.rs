use std::cmp;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::prefix_accumulate::PrefixSum;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i64();
    let a = read.vec_i64(n as usize);

    let mut nd = vec![None; n as usize];
    for i in (0..n as usize - 1).rev() {
        if a[i] == a[i + 1] {
            nd[i] = nd[i + 1];
        } else {
            nd[i] = Some(i as i64 + 1);
        }
    }

    let sum = PrefixSum::from_iter(a.iter().copied());
    let comb = |l: i64, r: i64| {
        if l == r {
            return -1;
        }
        if nd[l as usize].is_some_and(|x| x < r) {
            sum.get(l as u32..r as u32)
        } else {
            a[l as usize]
        }
    };
    for i in 0..n {
        let xx = cmp::max(comb(0, i), comb(i + 1, n));
        if xx <= a[i as usize] {
            emit!(write, -1);
            continue;
        }
        let mut l = 0;
        let mut r = n;
        while l < r {
            let m = (l + r) / 2;
            let x = cmp::max(
                comb(cmp::max(0, i - m), i),
                comb(i + 1, cmp::min(n, i + m + 1))
            );
            if x <= a[i as usize] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        emit!(write, r);
    }
    emitln!(write);
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
        4
        4
        3 2 4 2
        3
        1 2 3
        5
        2 2 3 1 1
        7
        4 2 3 6 1 1 8
        "), "\
        2 1 2 1
        1 1 -1
        2 1 -1 1 2
        2 1 1 3 1 1 4 ");
    }
}
