use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let x = read.digit_word();
    let mut k = read.usize();
    let mut dpositions = vec![vec![]; 10];
    for i in 0..x.len() {
        dpositions[x[i] as usize].push(i);
    }
    let mut dpos = dpositions.iter().map(|v| &v[..]).collect_vec();
    let mut p = 0;
    let mut ans = vec![];
    'outer: while k > 0 && p < x.len() {
        for dp in dpos.iter_mut() {
            while !dp.is_empty() && dp[0] < p {
                *dp = &dp[1..];
            }
        }
        let lower = if ans.is_empty() { 1 } else { 0 };
        for i in lower..=9 {
            if !dpos[i].is_empty() && dpos[i][0] - p <= k {
                ans.push(i as u32);
                k -= dpos[i][0] - p;
                p = dpos[i][0] + 1;
                continue 'outer;
            }
        }
        unreachable!();
    }
    if p < x.len() {
        assert_eq!(k, 0);
        ans.extend_from_slice(&x[p..]);
    } else {
        while k > 0 {
            ans.pop();
            k -= 1;
        }
    }
    emitln!(write, ans.iter().join(""));
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
        10000
        4
        1337
        0
        987654321
        6
        66837494128
        5
        7808652
        3
        "), "\
        1
        1337
        321
        344128
        7052");
        assert_trimmed_eq!(&run_solver(solve_case, "30001 2"), "300");
    }
}
