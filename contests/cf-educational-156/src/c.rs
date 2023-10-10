use std::iter;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

fn break_pos(pos: usize, n: usize) -> (usize, usize) {
    let mut start: usize = 0;
    for i in 0..n {
        let end = start + (n - i);
        if pos < end {
            return (i, pos - start);
        }
        start = end;
    }
    unreachable!();
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let s = read.word_as_chars();
    let pos = read.usize().from1b();

    let n = s.len();
    let (iiter, subp) = break_pos(pos, n);
    let mut first = 0;
    let mut nexts = (1..n).map(|x| Some(x)).chain(iter::once(None)).collect_vec();
    let mut prevs = iter::once(None).chain((0..n - 1).map(|x| Some(x))).collect_vec();

    let mut cur = first;
    let mut removed = 0;
    while removed < iiter {
        let Some(next) = nexts[cur] else {
            break;
        };
        if s[next] >= s[cur] {
            cur = next;
        } else {
            let prev = prevs[cur];
            if let Some(prev) = prev {
                nexts[prev] = Some(next);
            } else {
                first = next;
            }
            prevs[next] = prev;
            cur = prev.unwrap_or(next);
            removed += 1;
        }
    }

    let mut idx = first;
    for i in 0..subp {
        idx = nexts[idx].unwrap();
    }
    write!(write, "{}", s[idx]).unwrap();
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
        3
        cab
        6
        abcd
        9
        x
        1
        "), "abx");
        assert_trimmed_eq!(&run_solver(solve_case, "cabdd 8"), "d");
    }
}
