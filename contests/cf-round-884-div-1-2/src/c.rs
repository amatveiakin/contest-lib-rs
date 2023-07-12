use contest_lib_rs::relax::RelaxMinMax;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let c = read.vec_i64(n);
    let mut answer = i64::MIN;
    for k in 0..2 {
        let mut cc = c.iter().enumerate().filter(|(i, _)| i % 2 == k).map(|(_, &x)| x).collect::<Vec<_>>();
        let l = cc.len();
        if l == 0 {
            continue;
        }
        let max = *cc.iter().max().unwrap();
        let pos = cc.iter().position(|&x| x == max).unwrap();
        cc.remove(pos);
        let mut local_answer = max;
        local_answer += cc.iter().copied().filter(|&x| x > 0).sum::<i64>();
        answer.relax_max(local_answer);
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        6
        -3 1 4 -1 5 -9
        5
        998244353 998244353 998244353 998244353 998244353
        1
        -2718
        "), "\
        9
        2994733059
        -2718");
        assert_trimmed_eq!(&run_solver(solve_case, "5  1 -10 -100 -1000 1"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "4  10 1 10 100"), "101");
        assert_trimmed_eq!(&run_solver(solve_case, "3  7 10 7"), "14");
        assert_trimmed_eq!(&run_solver(solve_case, "3  3 10 3"), "10");
        assert_trimmed_eq!(&run_solver(solve_case, "5  -1 -1 -1 -1 -1"), "-1");
        // assert_trimmed_eq!(&run_solver(solve_case, ""), "");
    }
}
