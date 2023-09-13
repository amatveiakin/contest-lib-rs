use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils::Iterutils;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let s = read.digit_word();
    assert_eq!(s.len(), n);
    let mut m = 0;
    for i in 0..(n/2) {
        if s[i] != s[n-i-1] {
            m += 1;
        }
    }
    let t2 = s.iter().copied().sum::<u32>() % 2;
    let mut ans = vec![];
    for i in 0..=n {
        if n % 2 == 0 && i % 2 != t2 as usize {
            ans.push(0);
            continue;
        }
        if i.min(n - i) < m {
            ans.push(0);
        } else {
            ans.push(1);
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
        6
        101011
        5
        00000
        9
        100100011
        3
        100
        1
        1
        "), "\
        0010100
        111111
        0011111100
        0110
        11");
        assert_trimmed_eq!(&run_solver(solve_case, "4  0000"), "10101");
        assert_trimmed_eq!(&run_solver(solve_case, "4  0010"), "01010");
        assert_trimmed_eq!(&run_solver(solve_case, "4  0011"), "00100");
    }
}
