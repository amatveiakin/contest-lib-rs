use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

fn get_value(k: i64, bparam: i64, xparam: i64, cs: &[i64]) -> i64 {
    let mut ans: i64 = 0;
    for &c in cs {
        let x = c / k;
        let b = c % k;
        let a = k - b;
        assert_eq!(a * x + b * (x + 1), c);
        ans += x*x * (a*(a-1))/2 + (x+1)*(x+1) * (b*(b-1))/2 + x*(x+1)*a*b;
    }
    ans *= bparam;
    ans -= (k - 1) * xparam;
    ans
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, bparam, xparam] = read.i64s();
    let cs = read.vec_i64(n as usize);
    let maxc = *cs.iter().max().unwrap();
    let mut l = 1;
    let mut r = maxc;
    while r - l > 10 {
        let m1 = l + (r - l) / 3;
        let m2 = r - (r - l) / 3;
        let s1 = get_value(m1, bparam, xparam, &cs);
        let s2 = get_value(m2, bparam, xparam, &cs);
        if s1 < s2 {
            l = m1;
        } else {
            r = m2;
        }
    }
    l = (l - 100).max(1);
    r = (r + 100).min(maxc);
    let mut ans = 0;
    for k in l..=r {
        ans.relax_max(get_value(k, bparam, xparam, &cs));
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
        5
        3 1 0
        1 2 3
        3 5 10
        2 5 3
        4 3 3
        3 2 1 2
        4 1 0
        4 1 4 2
        4 1 10
        4 1 4 2
        "), "\
        4
        40
        9
        13
        0");
    }
}
