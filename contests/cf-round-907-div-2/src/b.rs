use std::array;

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, q] = read.usizes();
    let a = read.vec_u32(n);
    let x = read.vec_u32(q);
    let mut b: [_; 30] = array::from_fn(|_| Vec::new());
    for (p, &ap) in a.iter().enumerate() {
        for pow in (0..30).rev() {
            if ap % 2u32.pow(pow) == 0 {
                b[pow as usize].push((p, ap));
                break;
            }
        }
    }
    for xi in x {
        let add = 2u32.pow(xi - 1);
        for y in xi..30 {
            while let Some((a, ap)) = b[y as usize].pop() {
                b[xi as usize - 1].push((a, ap + add));
            }
        }
    }
    let mut ans = vec![0; n];
    for bb in b {
        for (p, ap) in bb {
            ans[p] = ap;
        }
    }
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
        4
        5 3
        1 2 3 4 4
        2 3 4
        7 3
        7 8 12 36 48 6 3
        10 4 2
        5 4
        2 2 2 2 2
        1 1 1 1
        5 5
        1 2 4 8 16
        5 2 3 4 1
        "), "\
        1 2 3 6 6
        7 10 14 38 58 6 3
        3 3 3 3 3
        1 3 7 11 19 ");
    }
}
