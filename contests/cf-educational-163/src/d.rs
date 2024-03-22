use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let s = read.word_as_chars();
    let n = s.len();
    for l in (1..=(n / 2)).rev() {
        let mut ok_since = None;
        for i in 0..(n - l) {
            let j = i + l;
            let ok = s[i] == s[j] || s[i] == '?' || s[j] == '?';
            if ok {
                if ok_since.is_none() {
                    ok_since = Some(i);
                }
                let start = ok_since.unwrap();
                if i - start + 1 >= l {
                    emitln!(write, l * 2);
                    return;
                }
            } else {
                ok_since = None;
            }
        }
    }
    emitln!(write, 0);
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
        assert_trimmed_eq!(&run_solver(solve_case, "?"), "0");
        assert_trimmed_eq!(&run_solver(solve_case, "??"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "???"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "????"), "4");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        zaabaabz
        ?????
        code?????s
        codeforces
        "), "\
        6
        4
        10
        0");
    }
}
