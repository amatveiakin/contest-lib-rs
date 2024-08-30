use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let s = read.word_as_chars();
    assert_eq!(s.len(), n);
    let mut l = CountingSet::from_item_iter(s.iter().copied());
    let mut t = n;
    let mut ans = String::new();
    while t > 0 {
        for ch in 'a'..='z' {
            if l.remove(ch) {
                ans.push(ch);
                t -= 1;
            }
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 3
        // abc
        // 5
        // edddf
        // 6
        // turtle
        // 8
        // pppppppp
        // 10
        // codeforces"), "\
        // acb
        // ddedf
        // urtlet
        // pppppppp
        // codeforces");
    }
}
