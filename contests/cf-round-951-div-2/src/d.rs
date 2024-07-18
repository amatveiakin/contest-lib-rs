use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

fn op(s: &[u32], p: usize) -> Vec<u32> {
    s[p..].iter().chain(s[..p].iter().rev()).copied().collect_vec()
}

fn check(s: &[u32], k: usize) -> bool {
    for i in 0..k {
        if s[i] != s[0] {
            return false;
        }
    }
    for i in 0..(s.len() - k) {
        if s[i] == s[i + k] {
            return false;
        }
    }
    true
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let s = read.word_as_digits();
    assert!(s.len() == n);
    assert!(n % k == 0);

    if k == n {
        if s.iter().all(|&x| x == s[0]) {
            emitln!(write, n);
        } else {
            emitln!(write, -1);
        }
        return;
    }

    let mut sl = vec![1; n];
    for i in 1..n {
        let j = i - 1;
        if s[i] == s[j] {
            sl[i] = sl[j] + 1;
        } else {
            sl[i] = 1;
        }
    }
    let mut sr = vec![1; n];
    for i in (0..n).rev() {
        let j = i + 1;
        if j < n && s[i] == s[j] {
            sr[i] = sr[j] + 1;
        } else {
            sr[i] = 1;
        }
    }
    // println!("###  s: {:?}", s);
    // println!("### sl: {:?}", sl);
    // println!("### sr: {:?}", sr);

    let mut ncand = 0;
    let start = sr.iter().position(|&x| x > k).unwrap_or(0).max(1);
    for i in start..n {
        if sr[i] == k && sl[i - 1] != k {
            let cs = op(&s, i);
            if check(&cs, k) {
                emitln!(write, i);
                return;
            }
            ncand += 1;
            if ncand >= 6 {
                break;
            }
        }
    }

    let mut ncand = 0;
    let start = sr.iter().position(|&x| x > k).unwrap_or(0).max(1);
    for i in start..n {
        if sr[i] == k {
            let cs = op(&s, i);
            if check(&cs, k) {
                emitln!(write, i);
                return;
            }
            ncand += 1;
            if ncand >= 2 {
                break;
            }
        }
    }
    emitln!(write, -1);
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
        // assert_trimmed_eq!(&run_solver(solve_case, "6 1  101001"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "2 1  01"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "2 1  00"), "-1");
        assert_trimmed_eq!(&run_solver(solve_case, "2 2  00"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "8 4  00001111"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "8 4  10000111"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "8 4  11000011"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "8 4  11100001"), "3");
        assert_trimmed_eq!(&run_solver(solve_case, "8 4  11110000"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "8 4  00000000"), "-1");
        assert_trimmed_eq!(&run_solver(solve_case, "9 3  000000000"), "-1");
        assert_trimmed_eq!(&run_solver(solve_case, "9 3  000011100"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "9 3  000001110"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "9 3  110001110"), "-1");
        assert_trimmed_eq!(&run_solver(solve_case, "18 3  111000111000100011"), "13");
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        8 4
        11100001
        4 2
        1110
        12 3
        111000100011
        5 5
        00000
        6 1
        101001
        8 4
        01110001
        12 2
        110001100110
        "), "\
        3
        -1
        7
        5
        4
        -1
        3");
    }
}
