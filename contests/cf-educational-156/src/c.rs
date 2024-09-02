use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::linked_vector::LinkedVector;

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
    let mut list = LinkedVector::new(n);

    let mut cur = 0;
    let mut removed = 0;
    while removed < iiter {
        let Some(next) = list.next(cur) else {
            break;
        };
        if s[next] >= s[cur] {
            cur = next;
        } else {
            let prev = list.prev(cur);
            list.remove(cur);
            cur = prev.unwrap_or(next);
            removed += 1;
        }
    }

    let mut idx = list.first().unwrap();
    for i in 0..subp {
        idx = list.next(idx).unwrap();
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
