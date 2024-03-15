// UNFINISHED

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let c = read.i64();
    let s_vec = read.vec_i64(n);

    let mut s_mod2 = [vec![], vec![]];
    let mut ban = 0i64;
    for s in s_vec {
        let p_sum = (s + 1).min(2 * c - s + 1).max(0);
        let p_diff = (c + 1 - s).max(0);
        let s_group = &mut s_mod2[(s % 2) as usize];
        let num_dup = match s_group.binary_search(&(2 * c - s)) {
            Ok(p) | Err(p) => p,
        };
        ban += p_sum + p_diff - num_dup as i64;
    }
    let ans = (c + 1) * (c + 2) / 2 - ban;
    // let ans = c * (c + 1) / 2 - ban;
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 8
        // 3 3
        // 1 2 3
        // 1 179
        // 57
        // 4 6
        // 0 3 5 6
        // 1 1
        // 1
        // 5 10
        // 0 2 4 8 10
        // 5 10
        // 1 3 5 7 9
        // 4 10
        // 2 4 6 7
        // 3 1000000000
        // 228 1337 998244353
        // "), "\
        // 3
        // 16139
        // 10
        // 2
        // 33
        // 36
        // 35
        // 499999998999122959");
    }
}
